use std::error::Error;
use std::process::Command;
use crate::core::device::{discover_keyboards, has_device_access};

pub fn user_in_input_group() -> bool {
    if let Ok(output) = Command::new("id").arg("-nG").output()
        && let Ok(s) = String::from_utf8(output.stdout) {
        return s.split_whitespace().any(|g| g == "input");
    }

    false
}

pub fn ensure_input_permissions() -> Result<(), Box<dyn Error>> {
    let devices = discover_keyboards();
    let has_real_device = devices
        .iter()
        .any(|device| !device.path.as_os_str().is_empty());

    if !has_real_device {
        return Ok(());
    }

    let has_accessible_device = devices
        .iter()
        .filter(|device| !device.path.as_os_str().is_empty())
        .any(|device| has_device_access(&device.path));

    if has_accessible_device {
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