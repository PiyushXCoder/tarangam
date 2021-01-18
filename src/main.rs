use gio::prelude::*;
use std::env::args;
use std::sync::{Arc, Mutex};

fn main() {
    let app = gtk::Application::new(Some("sng.tarangm"), Default::default())
    .expect("Failed to initiate gtk");

    let config = Arc::new(Mutex::new(tarangam::Config::new()));

    app.connect_activate(move |app| {
        let config = Arc::clone(&config);
        tarangam::build_ui(app, config);
    });

    app.run(&args().collect::<Vec<_>>());
}

