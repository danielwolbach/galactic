use crate::{config::Config, theme::Theme};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use std::str::FromStr;
use vte::prelude::*;

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<imp::Terminal>)
        @extends adw::Bin, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Terminal {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn apply_config(&self, config: &Config) {
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

        // Configure font.
        let font_description = gtk::pango::FontDescription::from_string(&config.general.font);
        terminal.set_font(Some(&font_description));

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
    }

    pub fn connect_child_exited<F: Fn() + 'static>(&self, callback: F) {
        self.imp().terminal.connect_child_exited(move |_, _| {
            callback();
        });
    }
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = "
        using Gtk 4.0;
        using Adw 1;
        using Vte 3.91;
        template $GalacticTerminal : Adw.Bin {
            Gtk.ScrolledWindow scrolled_window {
                Vte.Terminal terminal {}
            }
        }
    ")]
    pub struct Terminal {
        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,

        #[template_child]
        pub terminal: TemplateChild<vte::Terminal>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Terminal {
        const NAME: &'static str = "GalacticTerminal";
        type Type = super::Terminal;
        type ParentType = adw::Bin;

        fn class_init(class: &mut Self::Class) {
            class.bind_template();
        }

        fn instance_init(object: &glib::subclass::InitializingObject<Self>) {
            object.init_template();
        }
    }

    impl ObjectImpl for Terminal {
        fn constructed(&self) {
            self.parent_constructed();

            // Set up keyboard shortcuts.
            let event_key_controller = gtk::EventControllerKey::new();
            let terminal_copy = self.terminal.clone();
            event_key_controller.connect_key_pressed(move |_, key, _, modifier_type| {
                let shortcut_mask = gdk::ModifierType::CONTROL_MASK;
                if modifier_type.intersection(shortcut_mask).is_empty() {
                    return glib::Propagation::Proceed;
                }

                tracing::trace!(
                    "Pressed key with shortcut mask: `{}`",
                    key.name().unwrap_or_default()
                );

                match key.name().unwrap_or_default().as_str() {
                    "V" => {
                        tracing::debug!("Paste from clipboard.");
                        terminal_copy.emit_paste_clipboard();
                        terminal_copy.unselect_all();
                        glib::Propagation::Stop
                    }
                    "C" if terminal_copy.has_selection() => {
                        tracing::debug!("Copy selection to clipboard.");
                        terminal_copy.emit_copy_clipboard();
                        terminal_copy.unselect_all();
                        glib::Propagation::Stop
                    }
                    "plus" | "equal" => {
                        tracing::debug!("Scale font up.");
                        terminal_copy
                            .set_font_scale(10.0_f64.min(terminal_copy.font_scale() + 0.1));
                        glib::Propagation::Stop
                    }
                    "minus" | "underscore" => {
                        tracing::debug!("Scale font down.");
                        terminal_copy.set_font_scale(0.1_f64.max(terminal_copy.font_scale() - 0.1));
                        glib::Propagation::Stop
                    }
                    "0" => {
                        tracing::debug!("Reset font scale.");
                        terminal_copy.set_font_scale(1.0);
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            self.terminal.add_controller(event_key_controller);

            // Disable bell sound.
            self.terminal.set_audible_bell(false);
        }
    }

    impl WidgetImpl for Terminal {}

    impl BinImpl for Terminal {}
}
