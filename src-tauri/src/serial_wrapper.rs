use serialport::*;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{io, thread};
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// list the ports and return a vector of strings
pub fn list_ports() -> Vec<String> {
    // get the ports from serialport::available_ports()
    let ports = serialport::available_ports().expect("No ports found!");
    // make a vecotor of strings then create an iterator of ports then map port names an clone
    // and collect them into the vector
    let port_list: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
    // return the ports list
    return port_list;
}

// try to init the serial and return the port
pub fn init_port(app: tauri::AppHandle, port_name: &String, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    println!("init serial port");

    let port = serialport::new(port_name.as_str(), baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");


    // clone port
    let clone = port.try_clone().expect("Failed to clone");
    start_listen_clone(app, clone);

    // return port
    return Ok(port);
}

// clone the port and move it into the thread
fn start_listen_clone(app: tauri::AppHandle, mut clone: Box<dyn SerialPort>) {

    // try clone
    println!("port cloned");

    // serial buffer
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // move clone into thread
    thread::spawn(move || loop {
        match clone.read(serial_buf.as_mut_slice()) {
            Ok(size) => {
                let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                print!("{}", data_str);
                // emmit update to fronten
                app.emit_all("updateSerial", Payload {
                    message: data_str,
                }).unwrap();
            }
            // todo emmit_all on error
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    });
}

pub fn check_init(port: &mut Result<Box<dyn SerialPort>>) -> Result<&mut Box<dyn SerialPort>> {
    match port {
        Ok(p) => {
            return Ok(p);
        }
        Err(e) => {
            return Err(e.clone());
        }
    }
}

pub fn write_serial<'a>(
    port: &'a mut Result<Box<dyn SerialPort>>,
    input: &'a str,
) -> Result<usize> {
    // add newline
    let mut newinput = String::from(input);
    // newinput.push_str("\n");

    // try to write
    let output = newinput.as_bytes();
    // check if the port was initialized correctly
    match port {
        Ok(p) => match p.write(output) {
            Ok(w) => {
                return Ok(w);
            }
            Err(e) => {
                return Err(e.into());
            }
        },
        Err(e) => {
            return Err(e.clone());
        }
    }

    // port.write(output).expect("Write failed!");
}

pub fn wait_receive_serial(port: &mut Result<Box<dyn SerialPort>>) -> Result<String> {
    println!("waiting for response");

    match check_init(port) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    let timeout_ms = 10000;
    let start_time = Instant::now();

    loop {
        match receive_serial(port) {
            Ok(s) => {
                return Ok(s);
            }
            Err(_) => {}
        }

        // timeout
        if start_time.elapsed() >= Duration::from_millis(timeout_ms) {
            return Err(Error::new(
                serialport::ErrorKind::Unknown,
                "timeout exceeded",
            ));
        }

        // let str = receive_serial(port);
        // if str.len() != 0 {
        //     return str;
        // }
    }
}

pub fn receive_serial(port: &mut Result<Box<dyn SerialPort>>) -> Result<String> {
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // the port passed in was aviable
    match port {
        Ok(p) => match p.read(serial_buf.as_mut_slice()) {
            Ok(size) => {
                let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                return Ok(data_str);
            }
            Err(e) => {
                return Err(e.into());
            }
        },
        Err(e) => {
            return Err(e.clone());
        }
    }
}
