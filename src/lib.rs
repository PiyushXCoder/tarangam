mod graph;

use gtk::prelude::*;
use graph::{Graph, Line};

use std::sync::{Arc, Mutex};

use std::io::prelude::*;
use std::io::BufReader;

pub enum Status {
    JAGRIT, SAYAN, AVRODTIH, PARIVARTIT, NIKAS
}

pub struct Config {
    status: Status,
    bondrate: u32,
    port: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            status: Status::AVRODTIH,
            bondrate: 9600,
            port: "".to_owned()
        }
    }
}

enum Message {
    Msg(String),
    Status(String)
}

pub fn build_ui(app: &gtk::Application, config: Arc::<Mutex::<Config>>) {
    let builder = gtk::Builder::from_file("ui/main_window.glade");

    let win = builder.get_object::<gtk::ApplicationWindow>("win").expect("Resource file missing!");
    win.set_application(Some(app));
    let bar = builder.get_object::<gtk::Statusbar>("status_bar").expect("Resource file missing!");
    let log_area = builder.get_object::<gtk::TextView>("log_area").expect("Resource file missing!");

    let _ = Graph::new(
        builder.get_object::<gtk::DrawingArea>("draw_area").expect("Resource file missing!"),
        0.0, 100.0,
        0.0, 100.0,
        vec![
            Line::new(1.0,1.0,1.0,vec![(10.0,10.0),(20.0,20.0)]),
            Line::new(1.0,0.0,1.0,vec![(15.0,15.0),(50.0,25.0)]),
            Line::new(1.0,0.0,0.0,vec![(50.0,10.0),(70.0,60.0)])
        ]
    );

    win.show_all();

    // Bondrate
    let bondrate = builder.get_object::<gtk::ComboBoxText>("bondrate").expect("Resource file missing!");
    
    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    bondrate.connect_changed(move |cbx| {
        match tmp_config.lock() {
            Ok(mut config) => {
                config.bondrate = match cbx.get_active_text() {
                    Some(txt) => txt.to_string().parse::<u32>().unwrap_or(9600u32),
                    None => 9600
                };
            }, Err(_) => {
                tmp_bar.push(1, "Failed to change bondrate!");
            }
        }
    });

    // port
    let refresh_port = builder.get_object::<gtk::ToolButton>("refresh_port").expect("Resource file missing!");
    let port = builder.get_object::<gtk::ComboBoxText>("port").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_port = port.clone();
    refresh_port.connect_clicked(move |_| {
        tmp_port.remove_all();
        match serialport::available_ports() {
            Ok(ports) => {
                if ports.len() == 0 { tmp_bar.push(1, "No port found!"); }
                for p in ports {
                    tmp_port.append_text(p.port_name.as_str());
                }
            }, Err(_) => {
                tmp_bar.push(1, "No port found!");
            }
        }
    });

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    port.connect_changed(move |cbx| {
        match tmp_config.lock() {
            Ok(mut config) => {
                config.port = match cbx.get_active_text() {
                    Some(txt) => txt.to_string(),
                    None => "".to_owned()
                };
            }, Err(_) => {
                tmp_bar.push(1, "Failed to change port!");
            }
        }
    });

    //jagrit_btn
    let jagrit_btn = builder.get_object::<gtk::ToolButton>("jagrit_btn").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    jagrit_btn.connect_clicked(move |_ | {
        match tmp_config.lock() {
            Ok(mut config) => {
                tmp_bar.push(1, "Jagrit");
                config.status = Status::PARIVARTIT;
            }, Err(_) => {
                tmp_bar.push(1, "Failed to change port!");
            }
        }
    });

    //jagrit_btn
    let avrodith_btn = builder.get_object::<gtk::ToolButton>("avrodith_btn").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    avrodith_btn.connect_clicked(move |_| {
        match tmp_config.lock() {
            Ok(mut config) => {
                tmp_bar.push(1, "Avrodhit");
                config.status = Status::AVRODTIH;
            }, Err(_) => {
                tmp_bar.push(1, "Failed to change port!");
            }
        }
    });

    //clear_log
    let clear_log = builder.get_object::<gtk::ToolButton>("clear_log").expect("Resource file missing!");

    let tmp_log_area = log_area.clone();
    clear_log.connect_clicked(move |_| {
        tmp_log_area.get_buffer().expect("Couldn't get window").set_text("");
    });

    // send_entry
    let send_entry = builder.get_object::<gtk::Entry>("send_entry").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    send_entry.connect_activate(move |ent| {
        send_text(&tmp_config, ent, &tmp_bar);
    });

    //send_btn
    let send_btn = builder.get_object::<gtk::Button>("send_btn").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    send_btn.connect_activate(move |_| {
        send_text(&tmp_config, &send_entry, &tmp_bar);
    });

    // serial 
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let tmp_config = Arc::clone(&config);
    std::thread::spawn(move || {
        let mut bufread: Option<BufReader<Box<dyn  serialport::SerialPort>>> = None;
        let mut buf = String::new();
        loop {
            match tmp_config.lock() {
                Ok(mut config) => {
                    match config.status {
                        Status::AVRODTIH => {
                            bufread = None;
                            config.status = Status::SAYAN;
                        },
                        Status::JAGRIT => {
                            if let Some(read) = &mut bufread {
                                if let Ok(_) = read.read_line(&mut buf) {
                                    sender.send(Message::Msg(buf.clone())).unwrap();
                                    buf.clear();
                                }
                            }
                        },
                        Status::NIKAS => {},
                        Status::PARIVARTIT => {
                            let p = match serialport::new(&config.port, config.bondrate).open() {
                                Ok(p) => p,
                                Err(_) => {
                                    continue;
                                }
                            };
        
                            bufread = Some(BufReader::new(p));
                            config.status = Status::JAGRIT;
                        },
                        Status::SAYAN => {}
                    }

                }, Err(_) => {
                    sender.send(Message::Status("Faild prepare for communication!".to_owned())).unwrap();
                    return;
                }
            };
        }
    });

    let full_log = builder.get_object::<gtk::CheckButton>("full_log").expect("Resource file missing!");
    receiver.attach(None, move |msg| {
        match msg {
            Message::Msg(text) => {
                if !full_log.get_active() && text.starts_with("#") {

                } else {
                    let buf = log_area.get_buffer()
                        .expect("Couldn't get log_area");
                    buf.insert(&mut buf.get_end_iter(), &text);
                    log_area.scroll_to_iter(&mut buf.get_end_iter(), 0.4, true, 0.0, 0.0);
                }
            },
            Message::Status(text) => {
                bar.push(1, &text);
            }
        }
        glib::Continue(true)
    });
}

fn send_text(config: &Arc<Mutex<Config>>, entry: &gtk::Entry, bar: &gtk::Statusbar) {
    match config.lock() {
        Ok(config) => {
            let mut p = match serialport::new(&config.port, config.bondrate).open() {
                Ok(p) => p,
                Err(_) => {
                    bar.push(1, "Failed to change port!");
                    return;
                }
            };

            unsafe {
                p.write_all(entry.get_text().to_string().as_bytes_mut()).unwrap();
            }
            entry.set_text("");
        }, Err(_) => {
            bar.push(1, "Failed to change port!");
        }
    }
}
