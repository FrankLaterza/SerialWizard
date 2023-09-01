git use serialport::*;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
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
pub fn init_port(port_path: String, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    println!("init serial port");

    // state_gaurd.thread_handler = Some(crate::ThreadHandler { sender: sender, receiver: receiver });

    let port = serialport::new(port_path, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("no port");

    // return port
    return Ok(port);
}

// clone the port and move it into the thread
pub fn start_clone_thread(
    app: tauri::AppHandle,
    port_clone: Box<dyn SerialPort>,
    is_thead_open: Arc<AtomicBool>,
) {
    // clone port
    println!("port cloned");

    // state_gaurd.thread_handler = Some(ThreadHandler { sender: sender });
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // move clone into thread
    thread::spawn(move || {
            while is_thead_open.load(Ordering::Relaxed) {
                // error check before unwrap
                match port_clone.read(serial_buf.as_mut_slice()) {
                    Ok(size) => {
                        let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                        print!("{}", data_str);
                        // emmit update to fronten
                        app.emit_all("updateSerial", Payload { message: data_str })
                            .unwrap();
                    }
                    // todo emmit_all on error
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            println!("Terminating  thread.");
        });
}

// clone the port and move it into the thread
pub fn start_record_on_port(
    app: tauri::AppHandle,
    port_clone: Box<dyn SerialPort>,
    is_thead_open: Arc<AtomicBool>,
) {
    // clone port
    println!("port cloned");

    // state_gaurd.thread_handler = Some(ThreadHandler { sender: sender });
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // move clone into thread
    thread::spawn(move || {
            while is_thead_open.load(Ordering::Relaxed) {
                // error check before unwrap
                match port_clone.read(serial_buf.as_mut_slice()) {
                    Ok(size) => {
                        let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                        print!("{}", data_str);
                        // emmit update to fronten
                        app.emit_all("updateSerial", Payload { message: data_str })
                            .unwrap();


                        // record on file


                    }
                    // todo emmit_all on error
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                // todo emmit_all on error
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        println!("Terminating  thread.");
    });
}

// clone the port and move it into the thread
pub fn start_clone_thread_file(
    app: tauri::AppHandle,
    port_clone: Result<Box<dyn SerialPort>>,
    serial_thread_clone: Arc<AtomicBool>,
    file: crate::File,
) {
    // clone port
    println!("port cloned");

    // state_gaurd.thread_handler = Some(ThreadHandler { sender: sender });
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // todo check port clone success
    let mut clone = port_clone.unwrap();
    // move clone into thread
    thread::spawn(move || {
        // unclock file
        // loop through all data
        while serial_thread_clone.load(Ordering::Relaxed) {
            // error check before unwrap
            match clone.read(serial_buf.as_mut_slice()) {
                Ok(size) => {
                    let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();

                    println!("data in: {}", data_str);
                    file.write_all(data_str.as_bytes()).expect("Could not write to file");

                    // emmit update to fronten
                    app.emit_all("updateSerial", Payload { message: data_str })
                        .unwrap();
                }
                // todo emmit_all on error
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        println!("Terminating  thread.");
    });
}


pub fn write_serial<'a>(
    port: &'a mut Result<Box<dyn SerialPort>>,
    input: &'a str,
) -> Result<usize> {
    // add newline
    let newinput = String::from(input);
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
