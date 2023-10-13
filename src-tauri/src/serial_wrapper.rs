use serialport::*;
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
    // return tfhe ports list
    return port_list;
}

// try to init the serial and return the port
pub fn init_port(port_path: String, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    println!("Opening port: {}, baud: {}", port_path, baud_rate);
    let port = serialport::new(port_path, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    // return port
    return Ok(port);
}

// clone the port and move it into the thread
pub fn start_clone_thread(
    app: tauri::AppHandle,
    mut port_clone: Box<dyn SerialPort>,
    is_thread_open: Arc<AtomicBool>,
) {
    // state_gaurd.thread_handler = Some(ThreadHandler { sender: sender });
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // move clone into thread
    thread::spawn(move || {
        // open thread
        is_thread_open.store(true, Ordering::Relaxed);
        println!("Thread spawned");
        while is_thread_open.load(Ordering::Relaxed) {
            match port_clone.read(serial_buf.as_mut_slice()) {
                Ok(size) => {
                    let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                    println!("Received: {}", data_str);
                    // emmit update to fronten
                    app.emit_all("updateSerial", Payload { message: data_str })
                        .unwrap();
                }
                // todo emmit_all on error
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => {
                    // clone the app
                    let app_clone = app.clone();
                    use crate::AppData;
                    let state = app_clone.state::<AppData>();
                    // unclock gaurd
                    let mut state_gaurd = state.0.lock().unwrap();
                    // set the port as an none
                    // clone state gaurd data
                    let port_path = state_gaurd.port_items.port_path.clone();
                    let baud_rate = state_gaurd.port_items.baud_rate.clone();
                    // set the port as none
                    state_gaurd.port = None;

                    state_gaurd.is_recording = false;
                    is_thread_open.store(false, Ordering::Relaxed);

                    // let error_description = format!("{}{}", "An error occured opening port: ", e);
                    // rfd::MessageDialog::new()
                    //     .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                    //     .set_title("Port Error")
                    //     .set_description(error_description.as_str())
                    //     .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                    //     .show();
                }
            }
        }
        println!("Terminating no record thread and now enabling...");
        // reenable thread
        is_thread_open.store(true, Ordering::Relaxed);
    });
}

// clone the port and move it into the thread
pub fn start_record_on_port(
    app: tauri::AppHandle,
    mut port_clone: Box<dyn SerialPort>,
    is_thread_open: Arc<AtomicBool>,
    mut file: crate::File,
) {
    // state_gaurd.thread_handler = Some(ThreadHandler { sender: sender });
    let mut serial_buf: Vec<u8> = vec![0; 32];

    // move clone into thread
    thread::spawn(move || {
        // enable thread
        is_thread_open.store(true, Ordering::Relaxed);
        println!("Record thread spawned");
        while is_thread_open.load(Ordering::Relaxed) {
            match port_clone.read(serial_buf.as_mut_slice()) {
                Ok(size) => {
                    let data_str = String::from_utf8_lossy(&serial_buf[..size]).to_string();
                    println!("Recieved from record {}", data_str);
                    // emmit update to fronten
                    file.write_all(data_str.as_bytes())
                        .expect("Could not write to file");
                    // send event
                    app.emit_all("updateSerial", Payload { message: data_str })
                        .unwrap();
                }
                // todo emmit_all on error
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => {
                    // clone the app
                    let app_clone = app.clone();

                    use crate::AppData;
                    let state = app_clone.state::<AppData>();
                    // unclock gaurd
                    let mut state_gaurd = state.0.lock().unwrap();
                    // set the port as an none
                    // clone state gaurd data
                    let port_path = state_gaurd.port_items.port_path.clone();
                    let baud_rate = state_gaurd.port_items.baud_rate.clone();
                    // set the port as none
                    state_gaurd.port = None;
                    // toggle recording
                    state_gaurd.is_recording = false;
                    is_thread_open.store(false, Ordering::Relaxed);
                }
            }
        }
        println!("Terminating record thread and now enabling...");
        // reenable thread
        is_thread_open.store(true, Ordering::Relaxed);
    });
}

pub fn write_serial<'a>(port: &'a mut Box<dyn SerialPort>, input: &'a str) -> Result<usize> {
    // add newline
    let newinput = String::from(input);
    // newinput.push_str("\n");

    // try to write
    let output = newinput.as_bytes();
    // check if the port was initialized correctly

    port.write(output)?;

    return Ok(output.len());
}
