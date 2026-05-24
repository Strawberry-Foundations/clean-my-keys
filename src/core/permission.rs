use crate::core::device::has_device_access;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn user_in_input_group() -> bool {
    if let Ok(output) = Command::new("id").arg("-nG").output()
        && let Ok(s) = String::from_utf8(output.stdout) {
        return s.split_whitespace().any(|g| g == "input");
    }

    false
}

fn accessible_input_devices() -> Vec<PathBuf> {
    let mut devices = Vec::new();

    if let Ok(entries) = fs::read_dir("/dev/input/") {
        for entry in entries.flatten() {
            let path = entry.path();

            if path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("event"))
                && has_device_access(&path)
            {
                devices.push(path);
            }
        }
    }

    devices
}

pub fn ensure_input_permissions() -> Result<(), Box<dyn Error>> {
    if !accessible_input_devices().is_empty() {
        return Ok(());
    }

    karen::builder()
        .wrapper("pkexec")
        .with_env(&[
            "DISPLAY",
            "WAYLAND_",
            "XAUTHORITY",
            "DBUS_SESSION_BUS_ADDRESS",
            "XDG_RUNTIME_DIR",
        ])?;

    Ok(())
}