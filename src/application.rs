use crate::devices::{InputDevice, discover_keyboards};
use crate::devices::start_keyboard_lock;
use crate::fonts::{
    GSANSCODE_BOLD, ICON_KEYBOARD, ICON_KEYBOARD_LOCK, ICON_MOP, ICON_USB, icon, load_fonts,
};
use crate::theme::{button_style, container_style, pick_list_style};
use iced::widget::{Container, button, column, container, pick_list, row, text};
use iced::{Alignment, Fill, Font, Size, Theme};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Application {
    pub cleaning_enabled: bool,
    pub input_device: Option<InputDevice>,
    pub cleaning_signal: Arc<AtomicBool>,
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleCleaning,
    ChangeInputDevice(InputDevice),
    ChangeTheme(Theme),
}

impl Default for Application {
    fn default() -> Self {
        Self {
            cleaning_enabled: false,
            input_device: None,
            cleaning_signal: Arc::new(AtomicBool::new(false)),
            theme: Theme::Ferra,
        }
    }
}

impl Application {
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
            ..Default::default()
        }
    }

    /// # Panics
    /// Panics if panics
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleCleaning => {
                if self.cleaning_enabled {
                    self.cleaning_signal.store(false, Ordering::SeqCst);
                    self.cleaning_enabled = false;
                } else if let Some(input_device) = self.input_device.clone() {
                    if input_device.path.as_os_str().is_empty() {
                        return;
                    }

                    self.cleaning_signal.store(true, Ordering::SeqCst);

                    match start_keyboard_lock(
                        input_device.path.clone(),
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
                }
            }
            Message::ChangeInputDevice(input_device) => {
                if !self.cleaning_enabled {
                    self.input_device = Some(input_device);
                }
            }
            Message::ChangeTheme(theme) => {
                self.theme = theme;
            }
        }
    }

    /// # Panics
    /// Panics if panics
    pub fn view(&'_ self) -> Container<'_, Message> {
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

        container(
            column![
                pick_list(Theme::ALL, Some(self.theme.clone()), Message::ChangeTheme).style(pick_list_style),

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
                    .style(pick_list_style)
                ].align_y(Alignment::Center).spacing(16.0),

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
            .spacing(12.0),
        )
        .center(Fill)
    }
}
