extern crate queues;
use queues::*;
use serialport::available_ports;
use std::io::{self, Write};
use std::time::Duration;
pub fn list_serial_ports() -> String {
    let ports = available_ports().unwrap();
    let mut index = 0;
    let mut serial_ports: Vec<String> = Vec::new();
    for port in ports {
        println!("{:?}: {:?}", index, port.port_name);
        serial_ports.push(String::from(port.port_name));
        index = index + 1;
    }

    let mut port_number = String::new();
    println!("Please enter the selected port");
    io::stdin()
        .read_line(&mut port_number)
        .expect("Failed to read the port number\n");
    let port_num: usize = port_number.trim().parse().unwrap();
    let selected_port = String::from(serial_ports[port_num].clone());

    return selected_port;
}

pub fn read_from_port(port_name: String) {
    let open_port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(10))
        .open();

    match open_port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0;1000];
            println!("Connection successfully established\n");
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            println!("Failed to open port with Error: {:?}", e);
        }
    }

}


fn main() {
    println!("Hello, world!");
    let selected_port = list_serial_ports();
    println!("Selected port: {:?}", selected_port);
    read_from_port(selected_port);
}
