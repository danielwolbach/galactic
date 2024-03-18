use crate::{config::Config, constants, options::Options, theme::Theme, ui::window::Window};
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new(flags: &gio::ApplicationFlags) -> Self {
        let id = constants::APPLICATION_ID;
        tracing::info!("Create application with id `{id}`.");
        glib::Object::builder()
            .property("application-id", id)
            .property("flags", flags)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {}

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "GalacticApplication";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ApplicationImpl for Application {
        fn activate(&self) {
            let application = self.obj();
            let window = Window::new(&*application);

            let options = Options::parse();
            let config_path = options.config_path.unwrap_or(
                dirs::config_dir()
                    .unwrap()
                    .join(constants::APPLICATION_NAME.to_lowercase()),
            );

            let config = if options.default_config {
                tracing::info!("Use default config.");
                Config::default()
            } else {
                let config_path = config_path.join("config.toml");
                Config::load(&config_path).unwrap_or_else(|error| {
                    tracing::error!("Failed to get config from path {config_path:?}: {error}.");
                    tracing::info!("Use default config.");
                    Config::default()
                })
            };

            let theme = if let Some(theme) = &config.general.theme {
                let theme_path = config_path.join("themes").join(format!("{theme}.toml"));
                Theme::load(&theme_path).unwrap_or_else(|error| {
                    tracing::error!("Failed to get theme from path {theme_path:?}: {error}.");
                    tracing::info!("Use default theme.");
                    Theme::default()
                })
            } else {
                tracing::info!("Use default theme.");
                Theme::default()
            };

            window.apply_config(&config);
            window.apply_theme(&theme);

            window.present();
        }
    }

    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}
