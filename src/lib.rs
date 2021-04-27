/*
    This file is part of Tarangam.

    Tarangam is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Tarangam is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Tarangam.  If not, see <https://www.gnu.org/licenses/>
*/

//! Feel free to see through codes. Application is not written to be used as a library for other app. :)

pub mod graph;
pub mod util;
pub mod port_util;

use gtk::prelude::*;
use glib::clone;

use rand::Rng;

use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::BufReader;
use std::sync::atomic::Ordering;

use graph::Graph;
use util::Config;
use port_util as putil;

// Building and configuring GUI
pub fn build_ui(app: &gtk::Application, config: &Arc<Config>) {
    let builder = gtk::Builder::from_file(&config.ui_file);

    let win = builder.get_object::<gtk::ApplicationWindow>("win").expect("Resource file missing!");
    win.set_application(Some(app));

    // Status Bar
    let bar = builder.get_object::<gtk::Statusbar>("status_bar").expect("Resource file missing!");

    // Logging Area
    let log_area = builder.get_object::<gtk::TextView>("log_area").expect("Resource file missing!");

    // About Window
    let about_window = builder.get_object::<gtk::AboutDialog>("about_window").expect("Resource file missing!");

    // Save Window
    let save_window = builder.get_object::<gtk::FileChooserDialog>("save_window").expect("Resource file missing!");
    save_window.set_transient_for(Some(&win));
    save_window.set_action(gtk::FileChooserAction::Save);
    save_window.add_button("_Save", gtk::ResponseType::Apply);
    save_window.add_button("_Cancel", gtk::ResponseType::Cancel);

    let graph = Graph::new(
        builder.get_object::<gtk::DrawingArea>("draw_area").expect("Resource file missing!"),
        0.0, 100.0,
        0.0, 100.0,
        false,
        true,
        false,
        true,
        HashMap::new(),
        0.0
    );

    win.show_all();

    // required by signals
    let stambh_1 = builder.get_object::<gtk::Entry>("stambh_1").expect("Resource file missing!");
    let stambh_2 = builder.get_object::<gtk::Entry>("stambh_2").expect("Resource file missing!");
    let draw_baarik_box = builder.get_object::<gtk::CheckButton>("draw_baarik_box").expect("Resource file missing!");
    let port = builder.get_object::<gtk::ComboBoxText>("port").expect("Resource file missing!");
    let send_entry = builder.get_object::<gtk::Entry>("send_entry").expect("Resource file missing!");
    // Signals
    builder.connect_signals(|_, handler_name| {
        match handler_name {
            "exit_menu_activate" => Box::new(clone!(@weak win => @default-return None, move |_| {
                unsafe { win.destroy(); }
                None
            })),
            "about_menu_activate" => Box::new(clone!(@weak about_window => @default-return None, move |_| {
                about_window.show();
                about_window.present();
                None
            })),
            "save_menu_activate" => Box::new(clone!(@weak save_window => @default-return None, move |_| {
                save_window.show();
                save_window.present();
                None
            })),
            "gtk_main_quit" => Box::new(clone!(@weak save_window => @default-return None, move |_| {
                save_window.show();
                save_window.present();
                None
            })),
            "pankti_value_changed" => Box::new(clone!(@weak graph => @default-return None, move |a| {
                let btn = a[0].get::<gtk::SpinButton>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                tmp_graph.scale_x_size = btn.get_value();
                tmp_graph.redraw();
                None
            })),
            "stambh_1_changed" => Box::new(clone!(@weak graph => @default-return None, move |a| {
                let entry = a[0].get::<gtk::Entry>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                let val = entry.get_text().parse::<f64>().unwrap_or(0.0);
                let purana_y_start = tmp_graph.scale_y_start;
                let y_size = tmp_graph.scale_y_size;
                tmp_graph.scale_y_start = val;
                tmp_graph.scale_y_size = y_size + (purana_y_start - val);
                tmp_graph.redraw();
                None
            })),
            "stambh_2_changed" => Box::new(clone!(@weak graph => @default-return None, move |a| {
                let entry = a[0].get::<gtk::Entry>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                let val = entry.get_text().parse::<f64>().unwrap_or(0.0);
                let y_start = tmp_graph.scale_y_start;
                tmp_graph.scale_y_size = (val - y_start).abs();
                tmp_graph.redraw();
                None
            })),
            "nimna_stambh_toggled" => Box::new(clone!(@weak graph, @weak stambh_1, @weak stambh_2 => @default-return None, move |a| {
                let btn = a[0].get::<gtk::CheckButton>().unwrap().unwrap();
                graph.borrow_mut().auto_adjust_y = !btn.get_active();
                stambh_1.set_sensitive(btn.get_active());
                stambh_2.set_sensitive(btn.get_active());
                if btn.get_active() {
                    stambh_1.emit_activate();
                    stambh_2.emit_activate();
                }
                None
            })),
            "draw_patches_toggled" => Box::new(clone!(@weak graph => @default-return None, move |a| {
                let btn = a[0].get::<gtk::CheckButton>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                tmp_graph.draw_patch = btn.get_active();
                tmp_graph.redraw();
                None
            })),
            "draw_box_toggled" => Box::new(clone!(@weak graph, @weak draw_baarik_box => @default-return None, move |a| {
                let btn = a[0].get::<gtk::CheckButton>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                draw_baarik_box.set_sensitive(btn.get_active());
                tmp_graph.draw_box = btn.get_active();
                tmp_graph.redraw();
                None
            })),
            "draw_baarik_box_toggled" => Box::new(clone!(@weak graph => @default-return None, move |a| {
                let btn = a[0].get::<gtk::CheckButton>().unwrap().unwrap();
                let mut tmp_graph = graph.borrow_mut();
                tmp_graph.draw_baarik_box = btn.get_active();
                tmp_graph.redraw();
                None
            })),
            "clear_graph_clicked"=> Box::new(clone!(@weak graph => @default-return None, move |_| {
                let mut tmp_graph = graph.borrow_mut();
                tmp_graph.pankti_sankya = 0.0;
                tmp_graph.lines.clear();
                tmp_graph.redraw();
                None
            })),
            "bondrate_changed" => Box::new(clone!(@weak config => @default-return None, move |a| {
                let btn = a[0].get::<gtk::ComboBoxText>().unwrap().unwrap();
                config.bondrate.store(btn.get_active_text().unwrap().parse::<u32>().unwrap_or(9600u32), Ordering::SeqCst);
                None
            })),
            "port_changed" => Box::new(clone!(@weak config, @weak bar => @default-return None, move |a| {
                let btn = a[0].get::<gtk::ComboBoxText>().unwrap().unwrap();
                if let Some(val) = btn.get_active_text() {
                    match config.port.lock() {
                        Ok(mut a) => { *a = val.to_string() },
                        Err(_) => { bar.push(1, "Can't set Port"); }
                    }
                }
                None
            })),
            "refresh_port_clicked" => Box::new(clone!(@weak port, @weak bar, @weak config => @default-return None, move |_| {
                port.remove_all();
                match config.status.lock() {
                    Ok(mut a) => { *a = util::Status::AVRODTIH },
                    Err(_) => { bar.push(1, "Can't Avrodhit"); return None; }
                }
                bar.push(1, "Avrodhit");
                match serialport::available_ports() {
                    Ok(ports) => {
                        if ports.len() == 0 { bar.push(1, "No port found!"); }
                        for p in ports {
                            port.append_text(p.port_name.as_str());
                        }
                    }, Err(_) => {
                        bar.push(1, "No port found!");
                    }
                }
                None
            })),
            "jagrit_btn_clicked" => Box::new(clone!(@weak config, @weak graph, @weak bar => @default-return None, move |_| {
                let mut tmp_graph = graph.borrow_mut();
                tmp_graph.pankti_sankya = 0.0;
                tmp_graph.lines.clear();
                tmp_graph.redraw();
                bar.push(1, "Jagrit");
                match config.status.lock() {
                    Ok(mut a) => { *a = util::Status::PARIVARTIT },
                    Err(_) => { bar.push(1, "Can't Jagrit"); }
                }
                None
            })),
            "avrodith_btn_clicked" => Box::new(clone!(@weak config, @weak bar => @default-return None, move |_| {
                bar.push(1, "Avrodhit");
                match config.status.lock() {
                    Ok(mut a) => { *a = util::Status::AVRODTIH },
                    Err(_) => { bar.push(1, "Can't Avrodhit"); }
                }
                None
            })),
            "clear_log_clicked" => Box::new(clone!(@weak log_area => @default-return None, move |_| {
                log_area.get_buffer().expect("Couldn't get window").set_text("");
                None
            })),
            "send_entry_activate" => Box::new(clone!(@weak config, @weak bar => @default-return None, move |a| {
                let ent = a[0].get::<gtk::Entry>().unwrap().unwrap();
                putil::send_text(&config, &ent, &bar);
                None
            })),
            "send_btn_clicked" => Box::new(clone!(@weak config, @weak bar, @weak send_entry => @default-return None, move |_| {
                putil::send_text(&config, &send_entry, &bar);
                None
            })),
            "about_window_delete" => Box::new(|a| {
                let win = a[0].get::<gtk::AboutDialog>().unwrap().unwrap();
                win.hide();
                Some(true.to_value())
            }),
            "save_window_delete" => Box::new(|a| {
                let win = a[0].get::<gtk::FileChooserDialog>().unwrap().unwrap();
                win.hide();
                Some(true.to_value())
            }),
            "about_window_close" => Box::new(clone!(@weak about_window => @default-return None, move |_| {
                about_window.hide();
                None
            })),
            "save_window_close" => Box::new(clone!(@weak save_window => @default-return None, move |_| {
                save_window.hide();
                None
            })),
            _ => Box::new(|_| {None})
        }
    });

    let tmp_log_area = log_area.clone();
    let tmp_bar =  bar.clone();
    save_window.connect_response(move |win, res| {
        match res {
            gtk::ResponseType::Cancel => win.hide(),
            gtk::ResponseType::Apply => {
                if let Some(path) = win.get_filename() {
                    if let Some(buf) = tmp_log_area.get_buffer() {
                        let text = buf.get_text(&buf.get_start_iter(), &buf.get_end_iter(), false).unwrap().to_string();

                        match std::fs::write(path, text) {
                            Ok(_) => { 
                                win.hide();
                            },
                            Err(_) => {
                                tmp_bar.push(1, "Failed to save!");
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    });

    /*
        Thread to manage Serial Port

        The program runs a thread to read and parse the output from serial port and
        send it through mpsc (rx, tx) to a recever. Where it is added to Graph 
        or Log is added to text area or any status is displayed in bar
    */
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
    let tmp_config = Arc::clone(&config);
    tokio::task::spawn(async move {
        let mut bufread: Option<BufReader<Box<dyn  serialport::SerialPort>>> = None;
        let mut buf = String::new();
        loop {
            putil::serial_thread_work(&tmp_config, &mut bufread, &sender, &mut buf).await;
        }
    });

    // Reciver for MessageSerialThread from the "Thread to manage Serial Port" and works accordingly
    let full_log = builder.get_object::<gtk::CheckButton>("full_log").expect("Resource file missing!");
    let graph_data = builder.get_object::<gtk::TextView>("graph_data").expect("Resource file missing!");
    let tmp_graph = Rc::clone(&graph);
    receiver.attach(None, move |msg| {
        match msg {
            util::MessageSerialThread::Msg(text, msg_type) => {
                receiver_for_msg(text, &msg_type, &full_log, &log_area);
            },
            util::MessageSerialThread::Points(points) => {
                receiver_for_points(points, &tmp_graph, &graph_data);
            }
            util::MessageSerialThread::Status(text) => {
                bar.push(1, &text);
            }
        }
        glib::Continue(true)
    });
}

// Receives MessageSerialThread from Serial Port managing thread adds message to text area
fn receiver_for_msg(text: String, msg_type: &util::MessageSerialThreadMsgType, full_log: &gtk::CheckButton, log_area: &gtk::TextView) {
    if !full_log.get_active(){
        if let util::MessageSerialThreadMsgType::Point = msg_type {
            return;
        }
    }
    if text.len() <= 0 {
        return;
    }
    let buf = log_area.get_buffer()
        .expect("Couldn't get log_area");
    buf.insert(&mut buf.get_end_iter(), &format!("{}\n",text));
    log_area.scroll_to_iter(&mut buf.get_end_iter(), 0.4, true, 0.0, 0.0);
    log_area.queue_draw();
}

// Receives MessageSerialThread from Serial Port managing thread and add points to draw on graph
fn receiver_for_points(points: Vec<(String, f64)>, graph: &Rc<RefCell<Graph>>, graph_data: &gtk::TextView) {
    for (line, point) in points {
        let mut gp = graph.borrow_mut();
                
        let sankhya = gp.pankti_sankya;
        match gp.lines.get_mut(&line) {
            Some(val) => {
                val.points.push((sankhya, point));
            } None => {
                let v = vec![(sankhya, point)];
                let mut rng = rand::thread_rng();
                gp.lines.insert(line, graph::Line::new(rng.gen_range(0.0..1.0), 0.0, rng.gen_range(0.0..1.0), v));
                let buf = graph_data.get_buffer().expect("Couldn't get graph_data");
                buf.set_text("");
                gp.lines.iter().for_each(|(key, line)| {
                    buf.insert(&mut buf.get_end_iter(), "##");
                    
                    let tag = gtk::TextTag::new(None);
                    let rgba = gdk::RGBA {
                        red: line.color.0,
                        green: line.color.1,
                        blue: line.color.2,
                        alpha: 1.0
                    };
                    tag.set_property_background_rgba(Some(&rgba));
                    tag.set_property_foreground_rgba(Some(&rgba));
                    buf.get_tag_table().unwrap().add(&tag);
                    buf.apply_tag(&tag, &buf.get_iter_at_offset(buf.get_end_iter().get_offset() - 2), &buf.get_end_iter());
                    buf.insert(&mut buf.get_end_iter(), &format!(" {}, ", key));
                });
                graph_data.queue_draw();
            }
        }
        gp.redraw();
    }
    graph.borrow_mut().pankti_sankya += 1.0;
}
