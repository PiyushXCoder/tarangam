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

use std::sync::{atomic::*, Mutex};

/// Status of Serial reading
#[derive(Debug, Clone, Copy)]
pub enum Status {
    AVRODTIH, // Mode of being stopped
    SAYAN, // Mode of Sleeping
    JAGRIT, // Mode of Active
    PARIVARTIT // Mode of being values modified
}

#[derive(Debug)]
pub struct Config {
    pub ui_file: String,
    pub bondrate: AtomicU32,
    pub port: Mutex<String>,
    pub status: Mutex<Status>
}

/// For communication between mpsc of graph and serial port
#[derive(Debug)]
pub enum MessageSerialThread {
    Msg(String, MessageSerialThreadMsgType),
    Points(Vec<(String, f64)>),
    Status(String)
}

#[derive(Debug)]
pub enum MessageSerialThreadMsgType {
    Point,
    Log
}

impl Config {
    pub fn default() -> Self {
        let ui_file = std::env::var("TARANGAM_UI_FILE");
        
        Config {
            ui_file: match ui_file {
                Ok(val) => val, 
                Err(_) => std::env::current_exe().unwrap().parent().unwrap()
                    .join("ui.glade").to_str().unwrap().to_owned()
            },
            bondrate: AtomicU32::new(9600),
            port: Mutex::new(String::new()),
            status: Mutex::new(Status::AVRODTIH)
        }        
    }
}