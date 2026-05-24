#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::error::Error;
use crate::application::Application;
use crate::core::permission::ensure_input_permissions;
use crate::core::logging::set_verbose;

pub mod application;
pub mod core;
pub mod appearance;

fn main() -> Result<(), Box<dyn Error>> {
    let verbose = std::env::args().any(|argument| argument == "-v" || argument == "--verbose");
    
    set_verbose(verbose);
    ensure_input_permissions()?;

    iced::application(Application::default, Application::update, Application::view)
        .settings(Application::default_settings())
        .theme(|application: &Application| application.theme.clone())
        .subscription(Application::subscription)
        .title("Clean My Keys")
        .window(Application::default_window())
        .run()?;

    Ok(())
}
