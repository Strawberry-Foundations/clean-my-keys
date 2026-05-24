use std::{env, fs};
use std::path::PathBuf;
use iced::Theme;
use crate::appearance::theme::{theme_from_name};

pub fn config_directory() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
}

pub fn load_theme_from_config() -> Theme {
    fs::read_to_string(theme_config_path())
        .ok()
        .and_then(|content| theme_from_name(content.trim()))
        .unwrap_or(Theme::Ferra)
}

pub fn save_theme_to_config(theme: &Theme) {
    let theme_path = theme_config_path();

    if let Some(parent) = theme_path.parent()
        && let Err(error) = fs::create_dir_all(parent) {
        eprintln!("Failed to create config directory: {error}");
        return;
    }

    if let Err(error) = fs::write(&theme_path, theme.to_string()) {
        eprintln!("Failed to save theme: {error}");
    }
}

pub fn theme_config_path() -> PathBuf {
    config_directory().join("clean-my-keys").join("theme")
}
