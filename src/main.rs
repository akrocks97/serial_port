use serialport::available_ports;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
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

fn main() {
    println!("Hello, world!");
    let selected_port = list_serial_ports();
    println!("Selected port: {:?}", selected_port);
    let open_port = serialport::new(selected_port, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap();

    let port_access = Arc::new(Mutex::new(open_port));

    let read_mutex = Arc::clone(&port_access);
    let read_thread = thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        println!("Connection successfully established\n");
        loop {
            {
                let mut local_port = read_mutex.lock().unwrap();
                match (*local_port).read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let write_mutex = Arc::clone(&port_access);
    let write_thread = thread::spawn(move || loop {
        let mut std_input = String::new();
        io::stdin()
            .read_line(&mut std_input)
            .expect("Std input not working\n");
        println!("Input from user: {}", std_input);
        {
            let mut local_port = write_mutex.lock().unwrap();
            println!("Mutex lock acquired\n");
            match (*local_port).write(std_input.as_bytes()) {
                Err(e) => println!("failed due to error {:?}", e),
                Ok(n) => println!("Bytes written {}", n),
            };
        }
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}
