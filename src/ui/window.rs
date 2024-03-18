use crate::{config::Config, theme::Theme};
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn apply_config(&self, config: &Config) {
        tracing::debug!("Apply config `{config:?}`.");

        // Set window size.
        self.set_size_request(600, 400);
        self.set_default_size(
            config.window.size.width as i32,
            config.window.size.height as i32,
        );

        // Configure window title.
        self.set_title(Some(&config.window.title));

        // Set up close callback.
        let window_clone = self.clone();
        self.imp().terminal.connect_child_exited(move || {
            tracing::info!("Terminal child process exited. Close window.");
            window_clone.close();
        });

        // Apply config for terminal.
        self.imp().terminal.apply_config(config);
    }

    pub fn apply_theme(&self, theme: &Theme) {
        tracing::debug!("Apply theme `{theme:?}`.");

        // Set colors for header bar.
        // FIXME This way of applying custom CSS is deprecated and needs a replacement.
        let header_bar = self.imp().header_bar.get();
        let header_bar_css_provider = gtk::CssProvider::new();
        header_bar_css_provider.load_from_string(&format!(
            "headerbar {{ background-color: {}; color: {}; box-shadow: none; }}",
            theme.background, theme.foreground,
        ));
        #[allow(deprecated)]
        header_bar.style_context().add_provider(
            &header_bar_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Update color scheme.
        let style_manager = adw::StyleManager::default();
        style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

        // Apply theme for terminal.
        self.imp().terminal.apply_theme(theme);
    }
}

mod imp {
    use crate::ui::terminal::Terminal;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = "
    using Gtk 4.0;
    using Adw 1;
    using Vte 3.91;
    template $GalacticWindow : Adw.ApplicationWindow {
        Box {
            orientation: vertical;
            Adw.HeaderBar header_bar {}
            $GalacticTerminal terminal {
                vexpand: true;
            }
        }
    }
    ")]
    pub struct Window {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,

        #[template_child]
        pub terminal: TemplateChild<Terminal>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "GalacticWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(class: &mut Self::Class) {
            class.bind_template();
        }

        fn instance_init(object: &glib::subclass::InitializingObject<Self>) {
            object.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
