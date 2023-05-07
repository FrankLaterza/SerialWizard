use serialport::*;
use std::io::{Write};
use std::time::{Duration, Instant};

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
pub fn init_port(port_name: &String, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    println!("init serial port");

    let port = serialport::new(port_name.as_str(), baud_rate)
        .timeout(Duration::from_millis(10))
        .open();
    // // .expect("Failed to open port");

    // error handing
    match port {
        Ok(p) => {
            return Ok(p);
        }
        Err(e) => {
            println!("unable to initialize port: {:?}", e);
            return Err(e);
        }
    }
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
    newinput.push_str("\n");

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
