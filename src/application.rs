use iced::widget::{Container, button, column, container, pick_list, row, stack, text};
use iced::{Alignment, Fill, Font, Size, Theme};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::core::device::{InputDevice, discover_keyboards, has_device_access};
use crate::core::device::start_keyboard_lock;
use crate::appearance::fonts::{GSANSCODE_BOLD, ICON_ARROW_BACK, ICON_KEYBOARD, ICON_KEYBOARD_LOCK, ICON_MOP, ICON_SETTINGS, ICON_USB, icon, load_fonts};
use crate::appearance::theme::{button_style, container_style, pick_list_style, window_icon};
use crate::core::config::{load_theme_from_config, save_theme_to_config};
use crate::core::permission::user_in_input_group;

#[derive(Debug, Clone)]
pub struct Application {
    pub cleaning_enabled: bool,
    pub input_device: Option<InputDevice>,
    pub cleaning_signal: Arc<AtomicBool>,
    pub theme: Theme,
    pub settings_mode: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSettings,
    ToggleCleaning,
    ChangeInputDevice(InputDevice),
    ChangeTheme(Theme),
}

impl Default for Application {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Application {
    #[must_use]
    pub fn new(settings_mode: bool) -> Self {
        Self {
            cleaning_enabled: false,
            input_device: None,
            cleaning_signal: Arc::new(AtomicBool::new(false)),
            theme: load_theme_from_config(),
            settings_mode,
        }
    }
    #[must_use]
    pub fn default_settings() -> iced::Settings {
        iced::Settings {
            antialiasing: true,
            default_font: Font::with_name("Google Sans Code"),
            fonts: load_fonts(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn default_window() -> iced::window::Settings {
        iced::window::Settings {
            size: Size::new(600f32, 350f32),
            resizable: false,
            icon: window_icon(),
            ..Default::default()
        }
    }

    /// # Panics
    /// Panics if panics
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleSettings => {
                self.settings_mode = !self.settings_mode;
            }
            Message::ToggleCleaning => {
                if self.cleaning_enabled {
                    self.cleaning_signal.store(false, Ordering::SeqCst);
                    self.cleaning_enabled = false;
                } else if let Some(input_device) = self.input_device.clone() {
                    if input_device.path.as_os_str().is_empty() {
                        return;
                    }

                    if has_device_access(&input_device.path) {
                        self.cleaning_signal.store(true, Ordering::SeqCst);

                        match start_keyboard_lock(
                            &input_device.path,
                            Arc::clone(&self.cleaning_signal),
                        ) {
                            Ok(()) => {
                                self.cleaning_enabled = true;
                            }
                            Err(error) => {
                                self.cleaning_signal.store(false, Ordering::SeqCst);
                                eprintln!("{error}");
                            }
                        }
                    } else if user_in_input_group() {
                        eprintln!(
                            "User is in group 'input' but cannot open device. Check device permissions, re-login, or udev rules."
                        );
                    } else {
                        match karen::builder()
                            .wrapper("pkexec")
                            .with_env(&[
                                "DISPLAY",
                                "WAYLAND_",
                                "XAUTHORITY",
                                "DBUS_SESSION_BUS_ADDRESS",
                                "XDG_RUNTIME_DIR",
                            ])
                        {
                            Ok(_running_as) => {}
                            Err(err) => eprintln!("Failed to escalate privileges: {err}"),
                        }
                    }
                }
            }
            Message::ChangeInputDevice(input_device) => {
                if !self.cleaning_enabled {
                    self.input_device = Some(input_device);
                }
            }
            Message::ChangeTheme(theme) => {
                self.theme = theme;
                save_theme_to_config(&self.theme);
            }
        }
    }

    /// # Panics
    /// Panics if panics
    pub fn view(&'_ self) -> Container<'_, Message> {
        let header_icon = if self.settings_mode {
            ICON_ARROW_BACK
        } else {
            ICON_SETTINGS
        };

        let header = container(
            button(icon(header_icon))
                .on_press(Message::ToggleSettings)
                .style(button_style),
        )
        .width(Fill)
        .padding(16.0)
        .align_x(Alignment::End);

        let content = if self.settings_mode {
            self.view_settings()
        } else {
            self.view_main()
        };

        container(stack![container(content).center(Fill), header]).center(Fill)
    }

    fn view_main(&'_ self) -> Container<'_, Message> {
        let input_devices = discover_keyboards();

        let description = if self.cleaning_enabled {
            "Keyboard input is temporarily disabled. You can now safely clean your keys without triggering unwanted commands."
        } else {
            "Lock your keys for a quick wipe. Press the button to pause all keyboard inputs safely"
        };

        let keyboard_icon = if self.cleaning_enabled {
            ICON_KEYBOARD_LOCK
        } else {
            ICON_KEYBOARD
        };

        let description_container = container(
            row![icon(keyboard_icon).size(32.0), text(description).size(14.0)]
                .spacing(8.0)
                .align_y(Alignment::Center),
        )
        .style(|_| container_style())
        .padding(9.0)
        .width(550)
        .max_width(550);

        let content = column![
            icon(ICON_MOP).size(48.0),

            text("Clean My Keys").size(28.0).font(GSANSCODE_BOLD),

            description_container,

            row![
                row![icon(ICON_USB), text("Input Device")]
                    .align_y(Alignment::Center)
                    .spacing(4.0),
                pick_list(
                    input_devices,
                    self.input_device.as_ref(),
                    Message::ChangeInputDevice
                )
                .placeholder("None")
                .style(pick_list_style)
            ]
            .align_y(Alignment::Center)
            .spacing(16.0),

            if self.input_device.as_ref().is_some_and(|device| !device.path.as_os_str().is_empty()) {
                button(if self.cleaning_enabled { "Stop" } else { "Start" })
                    .on_press(Message::ToggleCleaning)
                    .style(button_style)
            } else {
                button(if self.cleaning_enabled { "Stop" } else { "Start" })
                    .style(button_style)
            }
        ]
        .align_x(Alignment::Center)
        .spacing(12.0);

        container(content)
    }

    fn view_settings(&'_ self) -> Container<'_, Message> {
        let settings_panel = container(
            column![
                row![
                    icon(ICON_SETTINGS).size(24.0),
                    text("Settings").size(24.0).font(GSANSCODE_BOLD)
                ].spacing(8.0),
                text("Theme").size(14.0),
                pick_list(Theme::ALL, Some(self.theme.clone()), Message::ChangeTheme)
                    .style(pick_list_style)
                    .width(220.0),
            ]
            .spacing(10.0)
            .align_x(Alignment::Start),
        )
        .style(|_| container_style())
        .padding(16.0)
        .width(320.0);

        container(settings_panel).center(Fill)
    }
}
