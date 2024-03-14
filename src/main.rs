mod config;
mod error;
mod theme;
mod utils;
mod window;

use adw::Application;
use config::Config;
use error::Result;
use gtk::prelude::*;
use window::Window;

fn main() {
    let application = Application::builder()
        .application_id("dev.danielwolbach.Galactic")
        .build();

    application.connect_activate(activate);
    application.run();
}

fn activate(application: &Application) {
    let window = setup(application);
    match window {
        Ok(window) => window.present(),
        Err(error) => {
            eprintln!("{error}");
            application.quit()
        }
    }
}

fn setup(application: &Application) -> Result<Window> {
    let config = Config::load_toml("config.toml")?;
    let window = Window::new(application, &config)?;
    Ok(window)
}
