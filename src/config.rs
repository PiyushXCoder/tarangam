use std::{cell::RefCell, rc::Rc};

use gio::prelude::*;
use glib::OptionFlags;

pub struct Config {
    pub ui_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui_file: std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("ui.glade")
                .to_str()
                .unwrap()
                .to_owned(),
        }
    }
}

#[allow(dead_code)]
impl Config {
    pub fn new(app: &gtk::Application) -> Rc<RefCell<Config>> {
        app.add_main_option(
            "ui_file",
            glib::Char::from(b'u'),
            OptionFlags::NONE,
            glib::OptionArg::String,
            "Path for ui file",
            Some("PATH"),
        );

        let conf = Rc::new(RefCell::new(Self::default()));

        let conf_clone = Rc::clone(&conf);
        app.connect_handle_local_options(move |_, dict| {
            if let Some(va) = dict.lookup_value("ui_file", None) {
                conf_clone.borrow_mut().ui_file = va.str().unwrap_or_default().to_owned();
            };
            -1
        });

        conf
    }
}
