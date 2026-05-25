use std::path::PathBuf;
use std::fmt;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDevice {
    pub name: String,
    pub path: PathBuf,
}

impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub use platform::{discover_keyboards, has_device_access, start_keyboard_lock};

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use evdev::{Device, KeyCode};
    use std::fs;
    use std::fs::OpenOptions;
    use std::path::Path;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::Duration;
    use crate::core::logging::log;

    pub fn has_device_access(path: &Path) -> bool {
        OpenOptions::new().read(true).open(path).is_ok()
    }

    pub fn discover_keyboards() -> Vec<InputDevice> {
        let mut keyboards = Vec::new();

        if let Ok(entries) = fs::read_dir("/dev/input/") {
            for entry in entries.flatten() {
                let path = entry.path();

                let Some(file_name) = path.file_name() else {
                    continue;
                };

                if !file_name.to_string_lossy().starts_with("event") {
                    continue;
                }

                match Device::open(&path) {
                    Ok(device) => {
                        match device.supported_keys() {
                            Some(keys) if keys.contains(KeyCode::KEY_A) => {
                                let name = device.name().unwrap_or("Unknown keyboard").to_string();
                                keyboards.push(InputDevice { name, path });
                            }
                            Some(_) => {
                                log(format!("Skipping {}: supported keys don't indicate a keyboard", path.display()));
                            }
                            None => {
                                log(format!("Skipping {}: device has no supported_keys()", path.display()));
                            }
                        }
                    }
                    Err(err) => {
                        log(format!("Failed to open {}: {err}", path.display()));
                    }
                }
            }
        }

        if keyboards.is_empty() {
            keyboards.push(InputDevice {
                name: String::from("No keyboard detected"),
                path: PathBuf::new(),
            });
        }

        keyboards
    }

    pub fn start_keyboard_lock(device_path: &Path, is_running: Arc<AtomicBool>) -> Result<(), String> {
        let mut device = Device::open(device_path)
            .map_err(|error| format!("Could not open device: {error}"))?;

        device
            .set_nonblocking(true)
            .map_err(|error| format!("Could not configure device: {error}"))?;

        let mut last_grab_error = None;
        for _ in 0..10 {
            match device.grab() {
                Ok(()) => {
                    last_grab_error = None;
                    break;
                }
                Err(error) if error.raw_os_error() == Some(16) => {
                    last_grab_error = Some(error);
                    thread::sleep(Duration::from_millis(25));
                }
                Err(error) => {
                    return Err(format!("Could not lock keyboard: {error}"));
                }
            }
        }

        if let Some(error) = last_grab_error {
            return Err(format!("Could not lock keyboard: {error}"));
        }

        thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                if let Err(error) = device.fetch_events()
                    && error.kind() != std::io::ErrorKind::WouldBlock {
                    log(format!("Failed to read keyboard events: {error}"));
                    break;
                }

                thread::sleep(Duration::from_millis(10));
            }

            let _ = device.ungrab();
        });

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use std::path::Path;
    use std::sync::atomic::Ordering;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::sync::OnceLock;
    use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, DispatchMessageW, GetMessageW, PeekMessageW, PM_REMOVE, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HHOOK, WH_KEYBOARD_LL, MSG};
    use windows::Win32::System::Threading::GetCurrentThreadId;

    static HOOK_HANDLE: OnceLock<Mutex<Option<HHOOK>>> = OnceLock::new();
    static RUNNING_FLAG: OnceLock<Mutex<Option<Arc<AtomicBool>>>> = OnceLock::new();

    pub fn has_device_access(_path: &Path) -> bool {
        true
    }

    pub fn discover_keyboards() -> Vec<InputDevice> {
        // Simplified: return a single default keyboard entry. Detailed enumeration
        // could be added later using Raw Input APIs.
        vec![InputDevice { name: String::from("Default keyboard (Windows)"), path: PathBuf::new() }]
    }

    extern "system" fn low_level_keyboard_proc(code: i32, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        unsafe {
            if code >= 0 {
                if let Some(m) = RUNNING_FLAG.get() {
                    if let Ok(guard) = m.lock() {
                        if let Some(flag) = &*guard {
                            if flag.load(Ordering::SeqCst) {
                                return LRESULT(1);
                            }
                        }
                    }
                }
            }
            CallNextHookEx(HHOOK(0), code, _wparam, _lparam)
        }
    }

    pub fn start_keyboard_lock(_device_path: &Path, is_running: Arc<AtomicBool>) -> Result<(), String> {
        let is_running_clone = Arc::clone(&is_running);

        // store running flag for hook to consult
        let running_slot = RUNNING_FLAG.get_or_init(|| Mutex::new(None));
        {
            let mut g = running_slot.lock().map_err(|_| "Mutex poisoned")?;
            *g = Some(Arc::clone(&is_running_clone));
        }

        let hook_slot = HOOK_HANDLE.get_or_init(|| Mutex::new(None));

        let handle = thread::spawn(move || {
            unsafe {
                let _thread_id = GetCurrentThreadId();
                let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), HINSTANCE(0), 0);
                if let Ok(mut slot) = hook_slot.lock() {
                    *slot = Some(hook);
                }

                let mut msg = MSG::default();
                // Use PeekMessage loop so we can poll `is_running` without being stuck.
                while is_running_clone.load(Ordering::SeqCst) {
                    while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                    thread::sleep(Duration::from_millis(10));
                }

                if let Ok(mut slot) = hook_slot.lock() {
                    if let Some(h) = *slot {
                        let _ = UnhookWindowsHookEx(h);
                        *slot = None;
                    }
                }
            }
        });

        // detach thread; it will exit when `is_running` becomes false
        handle.thread().unpark();

        Ok(())
    }
}
