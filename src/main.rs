#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::application::Application;
use crate::core::device::{discover_keyboards, has_device_access};
use std::error::Error;
use std::process::Command;

mod application;
mod core;
mod appearance;

fn user_in_input_group() -> bool {
    if let Ok(output) = Command::new("id").arg("-nG").output()
        && let Ok(groups) = String::from_utf8(output.stdout)
    {
        return groups.split_whitespace().any(|group| group == "input");
    }

    false
}

fn preflight_permission_check() {
    let devices = discover_keyboards();
    let has_real_device = devices
        .iter()
        .any(|device| !device.path.as_os_str().is_empty());

    if !has_real_device {
        return;
    }

    let has_accessible_device = devices
        .iter()
        .filter(|device| !device.path.as_os_str().is_empty())
        .any(|device| has_device_access(&device.path));

    if has_accessible_device {
        return;
    }

    if user_in_input_group() {
        eprintln!(
            "Preflight: Kein Zugriff auf erkannte Keyboard-Devices, obwohl der Benutzer in der Gruppe 'input' ist. Bitte udev-Regeln und Login-Session pruefen."
        );
    } else {
        eprintln!(
            "Preflight: Kein Zugriff auf erkannte Keyboard-Devices. Fuege den Benutzer zur Gruppe 'input' hinzu oder starte per pkexec."
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    preflight_permission_check();

    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(|application: &Application| application.theme.clone())
        .title("Clean My Keys")
        .window(Application::default_window())
        .run()?;

    Ok(())
}
