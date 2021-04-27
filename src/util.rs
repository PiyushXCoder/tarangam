
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