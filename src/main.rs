use adw::prelude::*;
use gtk::{gio, glib};
use tracing_subscriber::EnvFilter;
use ui::application::Application;

mod config;
mod constants;
mod options;
mod theme;
mod ui;

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let application = Application::new(&gio::ApplicationFlags::empty());

    static EMPTY_ARGS: Vec<String> = vec![];
    application.run_with_args(&EMPTY_ARGS)
}
