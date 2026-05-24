use evdev::{Device, KeyCode};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

pub fn discover_keyboards() -> Vec<InputDevice> {
    let mut keyboards = Vec::new();

    if let Ok(entries) = fs::read_dir("/dev/input/") {
        for entry in entries.flatten() {
            let path = entry.path();

            if path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("event")
            {
                if let Ok(device) = Device::open(&path) {
                    if device
                        .supported_keys()
                        .map_or(false, |keys| keys.contains(KeyCode::KEY_A))
                    {
                        let name = device.name().unwrap_or("Unknown keyboard").to_string();
                        keyboards.push(InputDevice { name, path });
                    }
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

pub fn start_keyboard_lock(device_path: PathBuf, is_running: Arc<AtomicBool>) -> Result<(), String> {
    let mut device = Device::open(&device_path)
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
