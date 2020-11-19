use gio::prelude::*;
use std::env::args;
use std::sync::Arc;

fn main() {
    let app = gtk::Application::new(Some("sng.trangm"), Default::default())
    .expect("Failed to initiate gtk");

    let config = Arc::new(trangam::Config::new());

    app.connect_activate(move |app| {
        let config = Arc::clone(&config);
        trangam::build_ui(app, config);
    });

    app.run(&args().collect::<Vec<_>>());
}

