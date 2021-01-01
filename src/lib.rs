mod graph;

use gtk::prelude::*;
use graph::{Graph, Line};

use std::sync::{Arc, Mutex};

pub enum Status {
    JAGRIT, SAYAN, AVRODTIH
}

pub struct Config {
    status: Status,
    bondrate: i32
}

impl Config {
    pub fn new() -> Config {
        Config {
            status: Status::AVRODTIH,
            bondrate: 9600
        }
    }
}

enum Message {
    UpdateLabel(String),
}

pub fn build_ui(app: &gtk::Application, config: Arc::<Mutex::<Config>>) {
    let builder = gtk::Builder::from_file("ui/main_window.glade");

    let win = builder.get_object::<gtk::ApplicationWindow>("win").expect("Resource file missing!");
    win.set_application(Some(app));
    let bar = builder.get_object::<gtk::Statusbar>("status_bar").expect("Resource file missing!");

    let _ = Graph::new(
        builder.get_object::<gtk::DrawingArea>("draw_area").expect("Resource file missing!"),
        0.0, 30.0,
        0.0, 50.0,
        vec![
            Line::new(1.0,1.0,1.0,vec![(10.0,10.0),(20.0,20.0)]),
            Line::new(1.0,0.0,1.0,vec![(15.0,15.0),(50.0,25.0)])
        ]
    );

    win.show_all();

    let bondrate = builder.get_object::<gtk::ComboBoxText>("log_area").expect("Resource file missing!");
    
    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    bondrate.connect_changed(move |cbx| {
        match tmp_config.lock() {
            Ok(mut config) => 
                config.bondrate = match cbx.get_active_text() {
                Some(txt) => txt.to_string().parse::<i32>().unwrap_or(9600),
                None => 9600 
            },
            Err(_) => {
                tmp_bar.push(1, "Failed to change bondrate!");
            }
        }
    });

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
    let tmpconfig = Arc::clone(&config);
    std::thread::spawn(move || {
        match tmpconfig.lock().unwrap().status {
            Status::JAGRIT => {
                sender.send(Message::UpdateLabel(String::from("jagrit"))).unwrap();
            }, Status::SAYAN => {
                
            }, Status::AVRODTIH => {
                sender.send(Message::UpdateLabel(String::from("avrodhit"))).unwrap();
            }
        }
    });


    let log_area = builder.get_object::<gtk::TextView>("log_area").expect("Resource file missing!");
    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateLabel(text) => {
                bar.push(1, &text);
                let buf = log_area.get_buffer()
                .expect("Couldn't get window");
                buf.insert(&mut buf.get_end_iter(), &text);
            },
        }
        glib::Continue(true)
    });
}

