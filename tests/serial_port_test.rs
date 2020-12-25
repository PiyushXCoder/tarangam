use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn start() {
    let ports = serialport::available_ports();
    println!("{:?}",ports);

    let p = serialport::open("/dev/ttyUSB0")
    .unwrap();

    let mut read = BufReader::new(p);
    let mut buf = String::new();
    loop {
        match read.read_line(&mut buf) {
            Ok(_) => 
            {
                print!("{} seconds", buf.parse::<f64>().unwrap() / 60.0);
                buf.clear();
            },
            Err(_) => {}
        }
    }
}
