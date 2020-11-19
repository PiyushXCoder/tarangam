mod graph;

use gtk::prelude::*;
use graph::Graph;

use std::sync::Arc;

pub enum Status {
    JAGRIT, SAYAN, AVRODTIH
}

pub struct Config {
    status: Status
}

impl Config {
    pub fn new() -> Config {
        Config {
            status: Status::AVRODTIH
        }
    }
}

enum Message {
    UpdateLabel(String),
}

pub fn build_ui(app: &gtk::Application, config: Arc::<Config>) {
    let builder = gtk::Builder::from_file("ui/main_window.glade");

    let win = builder.get_object::<gtk::ApplicationWindow>("win").expect("Resource file missing!");
    win.set_application(Some(app));

    let graph = Graph::new(
        builder.get_object::<gtk::DrawingArea>("draw_area").expect("Resource file missing!"),
        0.0, 100.0,
        0.0, 100.0,
        vec![(10.0,10.0),(20.0,20.0)]
    );

    win.show_all();

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
    let tmpconfig = Arc::clone(&config);
    std::thread::spawn(move || {
        match tmpconfig.status {
            Status::JAGRIT => {
                sender.send(Message::UpdateLabel(String::from("jagrit"))).unwrap();
            }, Status::SAYAN => {
                sender.send(Message::UpdateLabel(String::from("sayan"))).unwrap();
            }, Status::AVRODTIH => {
                sender.send(Message::UpdateLabel(String::from("avrodhit"))).unwrap();
            }
        }
    });

    let bar = builder.get_object::<gtk::Statusbar>("status_bar").expect("Resource file missing!");
    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateLabel(text) => {
                bar.push(1, &text);
            },
        }
        glib::Continue(true)
    });
}

