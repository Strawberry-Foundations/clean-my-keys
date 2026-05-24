#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::error::Error;

use crate::application::Application;
use crate::core::permission::ensure_input_permissions;

pub mod application;
pub mod core;
pub mod appearance;

fn main() -> Result<(), Box<dyn Error>> {
    ensure_input_permissions()?;

    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(|application: &Application| application.theme.clone())
        .title("Clean My Keys")
        .window(Application::default_window())
        .run()?;

    Ok(())
}
