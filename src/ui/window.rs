use crate::config::Config;
use crate::theme::Theme;
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk, gio, glib};
use std::str::FromStr;
use vte::prelude::*;

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
        tracing::debug!("Applying config `{config:?}`.");

        // Spawn terminal child process.
        let terminal = self.imp().terminal.get();
        terminal.spawn_async(
            vte::PtyFlags::DEFAULT,
            None,
            &config
                .general
                .command
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            &[],
            glib::SpawnFlags::DEFAULT,
            || {},
            -1,
            None::<&gio::Cancellable>,
            |_| {},
        );
        let window_clone = self.clone();
        terminal.connect_child_exited(move |_, _| {
            tracing::info!("Terminal child process exited. Closing window.");
            window_clone.close()
        });

        // Set window size.
        self.set_size_request(600, 400);
        self.set_default_size(
            config.window.size.width as i32,
            config.window.size.height as i32,
        );

        // Configure font.
        let font_description = gtk::pango::FontDescription::from_string(&config.general.font);
        terminal.set_font(Some(&font_description));

        // Configure window title.
        self.set_title(Some(&config.window.title));

        // Configure scroll bar.
        let scrolled_window = self.imp().scrolled_window.get();
        if !config.window.scroll_bar {
            scrolled_window.set_vscrollbar_policy(gtk::PolicyType::Never);
        }

        // Configure padding.
        // FIXME This way of applying custom CSS is deprecated and needs a replacement.
        let terminal_css_provider = gtk::CssProvider::new();
        terminal_css_provider.load_from_string(&format!(
            "vte-terminal {{ padding: {}px {}px; }}",
            config.window.padding.vertical, config.window.padding.horizontal
        ));
        #[allow(deprecated)]
        terminal.style_context().add_provider(
            &terminal_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn apply_theme(&self, theme: &Theme) {
        tracing::debug!("Applying theme `{theme:?}`.");

        // Translate colors to `gtk::gdk::RGBA` colors. Unwrapping is safe
        // because the theme deserializer checks for valid hex values.
        let foreground = gtk::gdk::RGBA::from_str(&theme.foreground).unwrap();
        let background = gtk::gdk::RGBA::from_str(&theme.background).unwrap();
        let palette: Vec<gtk::gdk::RGBA> = theme
            .palette
            .iter()
            .map(|c| gtk::gdk::RGBA::from_str(c).unwrap())
            .collect();

        // Set colors for terminal.
        let terminal = self.imp().terminal.get();
        terminal.set_colors(
            Some(&foreground),
            Some(&background),
            &palette.iter().collect::<Vec<_>>(),
        );

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
    }
}

mod imp {
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
            Gtk.ScrolledWindow scrolled_window {
                Vte.Terminal terminal {
                    hexpand: true;
                    vexpand: true;
                }
            }
        }
    }
    ")]
    pub struct Window {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,

        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,

        #[template_child]
        pub terminal: TemplateChild<vte::Terminal>,
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
            // Set up copy and paste.
            let event_key_controller = gtk::EventControllerKey::new();
            let terminal_copy = self.terminal.clone();
            event_key_controller.connect_key_pressed(move |_, key, _, modifier_type| {
                let mask = gdk::ModifierType::CONTROL_MASK.union(gdk::ModifierType::SHIFT_MASK);
                if !modifier_type.symmetric_difference(mask).is_empty() {
                    return glib::Propagation::Proceed;
                }

                match key.name().unwrap_or_default().to_lowercase().as_str() {
                    "v" => {
                        terminal_copy.emit_paste_clipboard();
                        terminal_copy.unselect_all();
                        glib::Propagation::Stop
                    }
                    "c" if terminal_copy.has_selection() => {
                        terminal_copy.emit_copy_clipboard();
                        terminal_copy.unselect_all();
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            self.terminal.add_controller(event_key_controller);

            self.parent_constructed();
        }
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
