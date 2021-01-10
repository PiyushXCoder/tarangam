mod graph;

use gtk::prelude::*;

use rand::Rng;

use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::io::prelude::*;
use std::io::BufReader;

use graph::Graph;

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

    let graph = Graph::new(
        builder.get_object::<gtk::DrawingArea>("draw_area").expect("Resource file missing!"),
        0.0, 100.0,
        0.0, 100.0,
        true,
        true,
        HashMap::new(),
        0.0
    );

    win.show_all();

    // exit_menu
    let exit_menu = builder.get_object::<gtk::MenuItem>("exit_menu").expect("Resource file missing!");
    let tmp_win = win.clone();
    exit_menu.connect_activate(move |_|{
        unsafe {
            tmp_win.destroy();
        }
    });

    // about_menu
    let about_menu = builder.get_object::<gtk::MenuItem>("about_menu").expect("Resource file missing!");
    let about = builder.get_object::<gtk::AboutDialog>("about").expect("Resource file missing!");
    
    about_menu.connect_activate(move |_|{
        about.show_all();
    });

    // save_log
    

    // pankti
    let pankti = builder.get_object::<gtk::SpinButton>("pankti").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    pankti.connect_value_changed(move |btn| {
        tmp_graph.borrow_mut().scale_x_size = btn.get_value();
        tmp_graph.borrow().area.queue_draw();
    });

    // stambh_1
    let stambh_1 = builder.get_object::<gtk::Entry>("stambh_1").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    stambh_1.connect_changed(move |entry| {
        let val = entry.get_text().parse::<f64>().unwrap_or(0.0);
        let purana_y_start = tmp_graph.borrow().scale_y_start;
        let y_size = tmp_graph.borrow().scale_y_size;
        tmp_graph.borrow_mut().scale_y_start = val;
        tmp_graph.borrow_mut().scale_y_size = y_size + (purana_y_start - val);
        tmp_graph.borrow().area.queue_draw();
    });

    // stambh_2
    let stambh_2 = builder.get_object::<gtk::Entry>("stambh_2").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    stambh_2.connect_changed(move |entry| {
        let val = entry.get_text().parse::<f64>().unwrap_or(0.0);
        let y_start = tmp_graph.borrow().scale_y_start;
        tmp_graph.borrow_mut().scale_y_size = (val - y_start).abs();
        tmp_graph.borrow().area.queue_draw();
    });

    // nimna_stambh
    let nimna_stambh = builder.get_object::<gtk::CheckButton>("nimna_stambh").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    nimna_stambh.connect_clicked(move |btn| {
        tmp_graph.borrow_mut().auto_adjust_y = !btn.get_active();
        stambh_1.set_sensitive(btn.get_active());
        stambh_2.set_sensitive(btn.get_active());
    });

    // draw_patches
    let draw_patches = builder.get_object::<gtk::CheckButton>("draw_patches").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    draw_patches.connect_clicked(move |btn| {
        tmp_graph.borrow_mut().draw_patch = btn.get_active();
        tmp_graph.borrow().area.queue_draw();
    });

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

    // clear_graph
    let clear_graph = builder.get_object::<gtk::ToolButton>("clear_graph").expect("Resource file missing!");

    let tmp_graph = Rc::clone(&graph);
    clear_graph.connect_clicked(move |_ | {
        tmp_graph.borrow_mut().pankti_sankya = 0.0;
        tmp_graph.borrow_mut().lines.clear();
        tmp_graph.borrow().area.queue_draw();
    });

    // jagrit_btn
    let jagrit_btn = builder.get_object::<gtk::ToolButton>("jagrit_btn").expect("Resource file missing!");

    let tmp_bar =  bar.clone();
    let tmp_config = Arc::clone(&config);
    let tmp_graph = Rc::clone(&graph);
    jagrit_btn.connect_clicked(move |_ | {
        match tmp_config.lock() {
            Ok(mut config) => {
                tmp_graph.borrow_mut().pankti_sankya = 0.0;
                tmp_graph.borrow_mut().lines.clear();
                tmp_graph.borrow().area.queue_draw();
                tmp_bar.push(1, "Jagrit");
                config.status = Status::PARIVARTIT;
            }, Err(_) => {
                tmp_bar.push(1, "Failed to change port!");
            }
        }
    });

    //avrodith_btn
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
    send_btn.connect_clicked(move |_| {
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
    let tmp_graph = Rc::clone(&graph);
    receiver.attach(None, move |msg| {
        match msg {
            Message::Msg(text) => {
                if text.starts_with("#") {
                    tmp_graph.borrow_mut().pankti_sankya += 1.0;
                    for (index, line) in text[1..].split(" ").enumerate() {
                        let part = line.split("=");   
                        let part = part.into_iter().collect::<Vec<&str>>();
                        if part.len() == 1 {
                            let num = match part[0].trim().parse::<f64>() {
                                Ok(val) => val,
                                Err(_) => {
                                    continue;
                                }
                            };
                            let mut gp = tmp_graph.borrow_mut();
                            
                            let sankhya = gp.pankti_sankya;
                            match gp.lines.get_mut(&index.to_string()) {
                                Some(val) => {
                                    val.points.push((sankhya, num));
                                } None => {
                                    let v = vec![(sankhya, num)];

                                    let mut rng = rand::thread_rng();
                                    gp.lines.insert(index.to_string(), graph::Line::new(rng.gen_range(0.0..1.0), 0.0, rng.gen_range(0.0..1.0), v));
                                }
                            }
                            gp.area.queue_draw();
                        } else if part.len() == 2 {
                            let num = match part[1].trim().parse::<f64>() {
                                Ok(val) => val,
                                Err(_) => {
                                    continue;
                                }
                            };
                            let mut gp = tmp_graph.borrow_mut();
                            
                            let sankhya = gp.pankti_sankya;
                            match gp.lines.get_mut(part[0]) {
                                Some(val) => {
                                    val.points.push((sankhya, num));
                                } None => {
                                    let v = vec![(sankhya, num)];

                                    let mut rng = rand::thread_rng();
                                    gp.lines.insert(part[0].to_owned(), graph::Line::new(rng.gen_range(0.0..1.0), 0.0, rng.gen_range(0.0..1.0), v));
                                }
                            }
                            gp.area.queue_draw();
                        }
                    }

                    if full_log.get_active(){
                        let buf = log_area.get_buffer()
                            .expect("Couldn't get log_area");
                        buf.insert(&mut buf.get_end_iter(), &text);
                        log_area.scroll_to_iter(&mut buf.get_end_iter(), 0.4, true, 0.0, 0.0);
                    }
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

    // Time ke hisab se pankti ko aage bhadhay
    // let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    // glib::timeout_add(300, move || {
    //     sender.send(()).unwrap();
    //     glib::Continue(true)
    // });

    // let tmp_graph = Rc::clone(&graph);
    // receiver.attach(None, move |_| {
    //     // println!("{:?}", tmp_graph.borrow_mut().lines[0].points);
    //     tmp_graph.borrow_mut().scale_x_start += 1.0;
    //     tmp_graph.borrow().area.queue_draw();
    //     glib::Continue(true)
    // });
}

fn send_text(config: &Arc<Mutex<Config>>, entry: &gtk::Entry, bar: &gtk::Statusbar) {
    match config.lock() {
        Ok(config) => {
            if let Status::JAGRIT = config.status {
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
            }
        }, Err(_) => {
            bar.push(1, "Failed to change port!");
        }
    }
}
