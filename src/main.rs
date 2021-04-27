
use gio::prelude::*;

use std::env::args;
use std::sync::Arc;

use tarangam::util::Config;


#[tokio::main]
async fn main() {
    let conf = Arc::new(Config::default());

    let app = gtk::Application::new(Some("sng.tarangm"), Default::default())
    .expect("Failed to initiate gtk");

    let tmp_conf = Arc::clone(&conf);
    app.connect_activate(move |app| {
        tarangam::build_ui(app, &tmp_conf);
    });

    app.run(&args().collect::<Vec<_>>());
}
