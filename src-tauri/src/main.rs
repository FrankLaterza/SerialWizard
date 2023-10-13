// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use chrono::Utc;
use serialport::SerialPort;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};
// todo move into wrapper
use rfd::FileDialog;
use std::fs::File;
use std::time::{Duration, SystemTime};
use chrono::prelude::*;
// todo move into Data struct
pub struct PortItmes {
    port_path: String,
    baud_rate: String,
}

// todo change Data name because its lame
// todo group data into sub structs or impl.
pub struct Data {
    // todo change to option
    port: Option<Box<dyn SerialPort>>,
    file_path: Option<PathBuf>,
    port_items: PortItmes,
    ending: String,
    is_thread_open: Arc<AtomicBool>,
    is_recording: bool,
    // todo track currect menu itmes
}

pub struct AppData(Mutex<Data>);

fn handle_serial_connect(app: tauri::AppHandle) {
    // clone the app
    let app_clone = app.clone();

    let state = app_clone.state::<AppData>();
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    // clone state gaurd data
    let port_path = state_gaurd.port_items.port_path.clone();
    let baud_rate = state_gaurd.port_items.baud_rate.clone();
    // convert baud to num
    let baud_rate_num = baud_rate.parse::<u32>().unwrap();

    // check port
    match &state_gaurd.port {
        // if port exists
        Some(_) => {
            // anounce killing the thread
            println!("Killing thread");
            // kill thread
            state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
            // wait for change
            while !state_gaurd.is_thread_open.load(Ordering::Relaxed) {}
            // set the port as an none
            state_gaurd.port = None;
        }
        // start new port
        None => {
            // start new port
            let port = serial_wrapper::init_port(port_path, baud_rate_num);
            // check port success
            match port {
                Ok(port) => {
                    // store report
                    let port_clone = port.try_clone().expect("Couldn't clone port");
                    // set the port
                    state_gaurd.port = Some(port);
                    // clone the thread handle (copys a refrence)
                    let is_thread_open_ref = state_gaurd.is_thread_open.clone();
                    // use clone on thread
                    serial_wrapper::start_clone_thread(app.clone(), port_clone, is_thread_open_ref);
                }
                Err(e) => {
                    let error_description = format!("{}{}", "An error occured opening port: ", e);
                    rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                        .set_title("Port Error")
                        .set_description(error_description.as_str())
                        .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                        .show();
                }
            }
        }
    }
}

// todo fix the most aweful nextest code you've ever seen
// kills current thread and starts recording
fn handle_start_record(app: tauri::AppHandle) {
    // clone the app
    let app_clone = app.clone();
    // get state from app
    let state = app_clone.state::<AppData>();
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    if !state_gaurd.is_recording {
        // check port
        match &state_gaurd.port {
            Some(port) => {
                // check if file exits
                match &state_gaurd.file_path {
                    Some(path) => {
                        // anounce killing the thread
                        println!("Killing thread");
                        // get the opened port
                        let port_clone = port.try_clone().unwrap();
                        // clone thread ref
                        let is_thread_open_ref = state_gaurd.is_thread_open.clone();
                        // Get the current system time
                        let system_time = SystemTime::now();
                        // Convert the system time to a DateTime object in the local timezone
                        let datetime: DateTime<Local> = system_time.into();
                        // Format the date and time as a string
                        let formatted_date_time = datetime.format("%Y-%m-%d_%H:%M:%S").to_string();
                        // format
                        let file_name = format!("SerialWizard_{}.txt", formatted_date_time);
                        // create file name
                        let file_path = path.join(file_name);

                        // create file
                        let file = File::create(&file_path);
                        match file {
                            Ok(file) => {
                                // kill thread
                                state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
                                // wait for change // todo add timout
                                while !state_gaurd.is_thread_open.load(Ordering::Relaxed) {}
                                // recording
                                state_gaurd.is_recording = true;
                                // start serial on port
                                serial_wrapper::start_record_on_port(
                                    app.clone(),
                                    port_clone,
                                    is_thread_open_ref,
                                    file,
                                );
                            }
                            Err(e) => {
                                state_gaurd.is_recording = false;
                                let error_description =
                                    format!("{}{}", "An error occured creating file: ", e);
                                rfd::MessageDialog::new()
                                    .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                                    .set_title("File Error")
                                    .set_description(error_description.as_str())
                                    .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                                    .show();
                            }
                        }
                    }
                    None => {
                        state_gaurd.is_recording = false;
                        rfd::MessageDialog::new()
                            .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                            .set_title("File Error")
                            .set_description("File path not set.")
                            .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                            .show();
                    }
                }
            }
            None => {
                state_gaurd.is_recording = false;
                // must be connected to serial port to start recording
                rfd::MessageDialog::new()
                    .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                    .set_title("Port Error")
                    .set_description("Connect to port first.")
                    .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                    .show();
            }
        }
    } else {
        // stop recording
        state_gaurd.port = None;
        // kill thread
        state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
        // wait for change // todo add timout
        while !state_gaurd.is_thread_open.load(Ordering::Relaxed) {}
        // unlock gaurd
        std::mem::drop(state_gaurd);
        // clone app and open port
        handle_serial_connect(app.clone());
    }
}

#[tauri::command]
fn get_ports() -> Vec<String> {
    return serial_wrapper::list_ports();
}

#[tauri::command]
fn send_serial(state: State<AppData>, input: String) {
    let mut state_gaurd = state.0.lock().unwrap();
    println!("writng string: {}", input);
    let input_format = format!("{}{}", input, state_gaurd.ending);
    match &mut state_gaurd.port {
        Some(port) => {
            let write = serial_wrapper::write_serial(port, input_format.as_str());
            match write {
                Ok(s) => {
                    println!("Write {} bytes success", s);
                }
                Err(e) => {
                    let error_description =
                        format!("{}{}", "An error occured writing to port: ", e);
                    rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                        .set_title("Write Error")
                        .set_description(error_description.as_str())
                        .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                        .show();
                }
            }
        }
        None => {
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                .set_title("Port Error")
                .set_description("Connect to port first.")
                .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                .show();
        }
    }
}

#[tauri::command]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// make a new window
#[tauri::command]
async fn make_window(handle: tauri::AppHandle) {
    tauri::WindowBuilder::new(&handle, "Setup", tauri::WindowUrl::App("/about".into()))
        .inner_size(500.0, 500.0)
        .resizable(false)
        .always_on_top(true)
        .title("Setup")
        .build()
        .unwrap();
}

fn main() {
    let baud_rates: Vec<&str> = vec![
        "300", "1200", "2400", "4800", "9600", "19200", "38400", "57600", "74880", "115200",
        "230400", "250000", "500000", "1000000", "2000000",
    ];
    let endings: Vec<&str> = vec!["\\n\\r", "\\n", "\\r", "None"];

    // tauri builder
    tauri::Builder::default()
        .manage(AppData(
            // create a new unintlized port
            Mutex::new(Data {
                port: None,
                file_path: Some(PathBuf::from("/home")),
                port_items: PortItmes {
                    port_path: String::from(""),
                    baud_rate: String::from("9800"),
                },
                ending: String::from(""),
                is_thread_open: Arc::new(AtomicBool::new(true)),
                is_recording: false,
            }),
        ))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_ports,
            send_serial,
            make_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
