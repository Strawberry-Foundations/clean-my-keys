use std::{env, fs};
use std::path::PathBuf;
use iced::Theme;
use crate::appearance::theme::{theme_from_name};
use crate::core::logging::log;

pub fn config_directory() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
}

#[must_use] 
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
        log(format!("Failed to create config directory: {error}"));
        return;
    }

    if let Err(error) = fs::write(&theme_path, theme.to_string()) {
        log(format!("Failed to save theme: {error}"));
    }
}

#[must_use] 
pub fn theme_config_path() -> PathBuf {
    config_directory().join("clean-my-keys").join("theme")
}
