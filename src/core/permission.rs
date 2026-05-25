use std::error::Error;
#[cfg(target_os = "linux")]
mod linux_permission {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::core::device::has_device_access;

    #[must_use]
    pub fn user_in_input_group() -> bool {
        if let Ok(output) = Command::new("id").arg("-nG").output()
            && let Ok(s) = String::from_utf8(output.stdout) {
            return s.split_whitespace().any(|g| g == "input");
        }

        false
    }

    pub fn accessible_input_devices() -> Vec<PathBuf> {
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

    /// # Errors
    /// Returns an error if no accessible input devices are found, indicating that the application likely lacks necessary permissions.
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
}

#[cfg(target_os = "windows")]
mod windows_permission {
    use super::*;

    #[must_use]
    pub fn user_in_input_group() -> bool {
        // Concept of 'input' group does not apply on Windows - assume allowed.
        true
    }

    pub fn accessible_input_devices() -> Vec<std::path::PathBuf> {
        // Device access model differs on Windows; return empty to trigger no escalation.
        Vec::new()
    }

    pub fn ensure_input_permissions() -> Result<(), Box<dyn Error>> {
        // No-op on Windows.
        Ok(())
    }
}

// Public wrappers
#[must_use]
pub fn user_in_input_group() -> bool {
    #[cfg(target_os = "linux")]
    return linux_permission::user_in_input_group();

    #[cfg(target_os = "windows")]
    return windows_permission::user_in_input_group();
}

pub fn accessible_input_devices() -> Vec<std::path::PathBuf> {
    #[cfg(target_os = "linux")]
    return linux_permission::accessible_input_devices();

    #[cfg(target_os = "windows")]
    return windows_permission::accessible_input_devices();
}

/// # Errors
pub fn ensure_input_permissions() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    return linux_permission::ensure_input_permissions();

    #[cfg(target_os = "windows")]
    return windows_permission::ensure_input_permissions();
}