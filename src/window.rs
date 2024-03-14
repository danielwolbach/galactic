use crate::{config::Config, error::Result, theme::Theme};
use adw::{subclass::prelude::*, Application};
use gtk::{
    gio,
    glib::{self, Object},
};
use std::str::FromStr;
use vte::{ApplicationExt, GtkWindowExt, PtyFlags, TerminalExt, TerminalExtManual, WidgetExt};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application, config: &Config) -> Result<Self> {
        let window: Self = Object::builder().property("application", app).build();

        // Spawn terminal with configured command.
        let terminal = window.imp().terminal.get();
        terminal.spawn_async(
            PtyFlags::DEFAULT,
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

        let app_clone = app.clone();
        terminal.connect_eof(move |_| {
            app_clone.quit();
        });

        // Configure size.
        {
            window.set_size_request(600, 400);
            window.set_default_size(
                config.window.size.width as i32,
                config.window.size.height as i32,
            );
        }

        // Configure everything else.
        window.configure(config)?;

        Ok(window)
    }

    pub fn configure(&self, config: &Config) -> Result<()> {
        #[allow(deprecated)]
        use vte::StyleContextExt;

        // Configure font.
        {
            let terminal = self.imp().terminal.get();
            let font_description = gtk::pango::FontDescription::from_string(&config.general.font);
            terminal.set_font(Some(&font_description));
        }

        // Configure theme.
        {
            // Load theme.
            let theme = if let Some(theme) = &config.general.theme {
                Theme::load_toml(format!("{theme}.toml"))?
            } else {
                Theme::default()
            };
            let foreground = gtk::gdk::RGBA::from_str(&theme.foreground)?;
            let background = gtk::gdk::RGBA::from_str(&theme.background)?;

            let palette: std::result::Result<Vec<gtk::gdk::RGBA>, _> = theme
                .palette
                .iter()
                .map(|c| gtk::gdk::RGBA::from_str(c))
                .collect();
            let palette = palette?;

            // Set for terminal.
            let terminal = self.imp().terminal.get();
            terminal.set_colors(
                Some(&foreground),
                Some(&background),
                &palette.iter().collect::<Vec<_>>(),
            );

            // Set for header bar.
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

        // Configure window title.
        {
            self.set_title(Some(&config.window.title));
        }

        // Configure scroll bar.
        {
            let scrolled_window = self.imp().scrolled_window.get();
            if !config.window.scroll_bar {
                scrolled_window.set_vscrollbar_policy(gtk::PolicyType::Never);
            }
        }

        // Configure padding.
        {
            let terminal = self.imp().terminal.get();
            let terminal_css_provider = gtk::CssProvider::new();
            terminal_css_provider.load_from_string(&format!(
                "vte-terminal {{ padding: {}px {}px; }}",
                config.window.padding.y, config.window.padding.x
            ));
            #[allow(deprecated)]
            terminal.style_context().add_provider(
                &terminal_css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        Ok(())
    }
}

mod imp {
    use adw::{subclass::prelude::*, HeaderBar};
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate, ScrolledWindow};
    use vte::Terminal;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(string = "
    using Gtk 4.0;
    using Adw 1;
    using Vte 2.91;
    template $GalacticApplicationWindow : Adw.ApplicationWindow {
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
        pub header_bar: TemplateChild<HeaderBar>,

        #[template_child]
        pub scrolled_window: TemplateChild<ScrolledWindow>,

        #[template_child]
        pub terminal: TemplateChild<Terminal>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "GalacticApplicationWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(class: &mut Self::Class) {
            class.bind_template();
        }

        fn instance_init(object: &InitializingObject<Self>) {
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
