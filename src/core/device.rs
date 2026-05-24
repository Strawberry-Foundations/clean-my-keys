use evdev::{Device, KeyCode};
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::core::logging::log;

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct InputDevice {
    pub name: String,
    pub path: PathBuf,
}

impl std::fmt::Display for InputDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn has_device_access(path: &Path) -> bool {
    OpenOptions::new().read(true).open(path).is_ok()
}

pub fn discover_keyboards() -> Vec<InputDevice> {
    let mut keyboards = Vec::new();

    if let Ok(entries) = fs::read_dir("/dev/input/") {
        for entry in entries.flatten() {
            let path = entry.path();

            if !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("event")
            {
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
        .grab()
        .map_err(|error| format!("Could not lock keyboard: {error}"))?;

    thread::spawn(move || {
        while is_running.load(Ordering::SeqCst) {
            let _ = device.fetch_events();
            thread::sleep(Duration::from_millis(10));
        }

        let _ = device.ungrab();
    });

    Ok(())
}
