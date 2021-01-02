use std::io::prelude::*;
use std::io::BufReader;
use std::time::Duration;

#[test]
fn start() {
    let ports = serialport::available_ports();
    println!("{:?}",ports);

    let mut p = serialport::new("/dev/ttyUSB1", 9600).timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    unsafe {
        p.write_all("buf".to_owned().as_bytes_mut()).unwrap();
    }
    let mut read = BufReader::new(p);
    let mut buf = String::new();
    loop {
        match read.read_line(&mut buf) {
            Ok(_) => 
            {
                print!("{}", buf);
                buf.clear();
            },
            Err(_) => {}
        }
    }
}
