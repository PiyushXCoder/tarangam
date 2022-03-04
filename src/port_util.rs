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

use gtk::prelude::*;

use std::io::prelude::*;
use std::io::BufReader;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::{util, util::Properties};

// Controls the thread and read from serial port
pub(crate) async fn serial_thread_work(
    config: &Arc<Properties>,
    bufread: &mut Option<BufReader<Box<dyn serialport::SerialPort>>>,
    sender: &glib::Sender<util::MessageSerialThread>,
    buf: &mut String,
) {
    let status = match config.status.try_lock() {
        Ok(a) => a.to_owned(),
        Err(_) => {
            return;
        }
    };

    match status {
        util::Status::AVRODTIH => {
            *bufread = None;
            match config.status.lock() {
                Ok(mut a) => *a = util::Status::SAYAN,
                Err(_) => {
                    return;
                }
            };
        }
        util::Status::JAGRIT => {
            if let Some(read) = bufread {
                if let Ok(_) = read.read_line(buf) {
                    for line in buf.lines() {
                        if line.len() == 0 {
                            continue;
                        } else if line.starts_with("#") {
                            let mut points: Vec<(String, f64)> = Vec::new();
                            for (index, line) in line[1..].split(" ").enumerate() {
                                let part = line.split("=");
                                let part = part.into_iter().collect::<Vec<&str>>();
                                if part.len() == 1 {
                                    let num = match part[0].trim().parse::<f64>() {
                                        Ok(val) => val,
                                        Err(_) => {
                                            continue;
                                        }
                                    };

                                    points.push((index.to_string(), num));
                                } else if part.len() == 2 {
                                    points.push((
                                        part[0].trim().to_owned(),
                                        part[1].parse::<f64>().unwrap(),
                                    ));
                                }
                            }
                            sender
                                .send(util::MessageSerialThread::Points(points))
                                .unwrap();
                            sender
                                .send(util::MessageSerialThread::Msg(
                                    line.to_owned(),
                                    util::MessageSerialThreadMsgType::Point,
                                ))
                                .unwrap();
                        } else {
                            sender
                                .send(util::MessageSerialThread::Msg(
                                    line.to_owned(),
                                    util::MessageSerialThreadMsgType::Log,
                                ))
                                .unwrap();
                        }
                    }
                    buf.clear();
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        util::Status::PARIVARTIT => {
            let port = match config.port.lock() {
                Ok(a) => a.to_owned(),
                Err(_) => {
                    return;
                }
            };
            let p = match serialport::new(&port, config.bondrate.load(Ordering::SeqCst)).open() {
                Ok(p) => p,
                Err(_) => {
                    return;
                }
            };

            *bufread = Some(BufReader::new(p));
            match config.status.try_lock() {
                Ok(mut a) => *a = util::Status::JAGRIT,
                Err(_) => {
                    return;
                }
            };
        }
        _ => {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}

// // Sends text through Serial Post to device
pub(crate) fn send_text(config: &Arc<Properties>, entry: &gtk::Entry, bar: &gtk::Statusbar) {
    let status = match config.status.try_lock() {
        Ok(a) => a.to_owned(),
        Err(_) => {
            return;
        }
    };
    if let util::Status::JAGRIT = status {
        let port = match config.port.lock() {
            Ok(a) => a.to_owned(),
            Err(_) => {
                bar.push(1, "Failed to set port!");
                return;
            }
        };

        let mut p = match serialport::new(port, config.bondrate.load(Ordering::SeqCst)).open() {
            Ok(p) => p,
            Err(_) => {
                bar.push(1, "Failed to change port!");
                return;
            }
        };

        unsafe {
            p.write_all(entry.text().to_string().as_bytes_mut())
                .unwrap();
        }
        entry.set_text("");
    }
}
