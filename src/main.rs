use adw::prelude::*;
use gtk::{
    gio::{self},
    glib,
};
use tracing_subscriber::EnvFilter;
use ui::application::Application;

mod config;
mod constants;
mod options;
mod theme;
mod ui;

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("GALACTIC_LOG"))
        .init();

    let application = Application::new(&gio::ApplicationFlags::NON_UNIQUE);

    static EMPTY_ARGS: Vec<String> = vec![];
    application.run_with_args(&EMPTY_ARGS)
}
