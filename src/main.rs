mod config;
mod error;
mod theme;
mod utils;
mod window;

use adw::Application;
use config::Config;
use gtk::prelude::*;
use window::Window;

fn main() {
    let application = Application::builder()
        .application_id("io.github.danielwolbach.Galactic")
        .build();

    application.connect_activate(activate);
    application.run();
}

fn activate(application: &Application) {
    let config = Config::load_toml("config.toml").unwrap_or_else(|error| {
        eprintln!("Failed to load configuration: {error}");
        Config::default()
    });

    let window = Window::new(application, &config);

    match window {
        Ok(window) => window.present(),
        Err(error) => eprintln!("Failed to initialize window: {error}"),
    };
}
