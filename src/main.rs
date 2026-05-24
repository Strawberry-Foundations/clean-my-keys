#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::application::Application;
use std::error::Error;

mod application;
mod devices;
mod fonts;
mod theme;

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(|application: &Application| application.theme.clone())
        .title("Clean My Keys")
        .window(Application::default_window())
        .run()?;

    Ok(())
}
