// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use serialport::SerialPort;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{Manager, State};
// todo move into wrapper
use rfd::FileDialog;
use std::fs::File;
use std::time::SystemTime;
use chrono::{DateTime, Local};

// todo move into Data struct
pub struct PortItems {
    port_path: String,
    baud_rate: u32,
    ending: String,
}

// todo change Data name because its lame
// todo group data into sub structs or impl.
pub struct Data {
    // todo change to option
    port: Option<Box<dyn SerialPort>>,
    folder_path: Option<PathBuf>,
    port_items: PortItems,
    is_thread_open: Arc<AtomicBool>,
    is_recording: bool,
    // todo track currect menu itmes
}

pub struct AppData(Mutex<Data>);

#[tauri::command]
fn set_port_items(state: State<AppData>, port: &str, baud: &str, ending: &str){
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();
    
    // store port items
    // TODO change ending to update without port init
    state_gaurd.port_items = PortItems {
        port_path: port.to_string(),
        baud_rate: baud.to_string().parse::<u32>().unwrap(),
        ending: ending.to_string()
    };
}

#[tauri::command]
fn handle_serial_connect(app: tauri::AppHandle) -> bool {

    // clone the app
    let app_clone = app.clone();
    // get the state
    let state = app_clone.state::<AppData>();
    // unlock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    // check if recording
    if state_gaurd.is_recording {
        rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
        .set_title("Port Error")
        .set_description("Please stop recording before disconnecting")
        .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
        .show();
        return true;
    }


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
            return false;
        }
        // start new port
        None => {
            // start new port TODO make it not really long
            let port = serial_wrapper::init_port(state_gaurd.port_items.port_path.to_string(), state_gaurd.port_items.baud_rate);
            // check port success
            match port {
                Ok(port) => {
                    // store report
                    let port_clone = port.try_clone().expect("Couldn't clone port");
                    // store the port
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

                    return false;
                }
            }
        }
    }
    return true;
}

// TODO fix the most aweful nested code you've ever seen
// kills current thread and starts recording
#[tauri::command]
fn handle_start_record(app: tauri::AppHandle) -> bool {
    // clone the app
    let app_clone = app.clone();
    // get state from app
    let state = app_clone.state::<AppData>();
    // unlock gaurd
    let mut state_gaurd = state.0.lock().unwrap();
    println!("start handle record");
    if !state_gaurd.is_recording {
        // check port
        match &state_gaurd.port {
            Some(port) => {
                // check if file exits
                match &state_gaurd.folder_path {
                    Some(path) => {
                        // get the opened port
                        let port_clone = port.try_clone().unwrap();
                        // clone thread ref
                        let is_thread_open_ref = state_gaurd.is_thread_open.clone();
                        // get the current system time
                        let system_time = SystemTime::now();
                        // convert the system time to a DateTime object in the local timezone
                        let datetime: DateTime<Local> = system_time.into();
                        // format the date and time as a string
                        let formatted_date_time = datetime.format("%Y-%m-%d_%H.%M.%S").to_string();
                        // format
                        let file_name = format!("SerialWizard_{}.txt", formatted_date_time);
                        // create file name
                        let file_path = path.join(file_name);

                        let path_clone = path.clone();
                        // create file
                        let file = File::create(&file_path);
                        match file {
                            Ok(file) => {
                                // kill thread
                                state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
                                // wait for change // TODO add timRout
                                while !state_gaurd.is_thread_open.load(Ordering::Relaxed) {}
                                // recording
                                state_gaurd.is_recording = true;
                                // start serial on port
                                serial_wrapper::start_record_on_port(
                                    app.clone(),
                                    port_clone,
                                    is_thread_open_ref,
                                    Some(file),
                                    path_clone
                                );
                                println!("finish start clone");
                                return true;
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
                                return false;
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
                        return false;
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
                return false;
            }
        }
    }
    // stop recording
    state_gaurd.is_recording = false;
    // set port to none
    state_gaurd.port = None;
    // kill thread
    state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
    // wait for change // TODO add timeout
    while !state_gaurd.is_thread_open.load(Ordering::Relaxed) {}
    // drop the state_gaurd to restart serial connect
    std::mem::drop(state_gaurd);
    handle_serial_connect(app.clone());
    return false;
}

#[tauri::command]
fn set_folder_path(state: State<AppData>){
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();
    // set the folder path
    let dir = FileDialog::new().set_directory("/").pick_folder();
    // store the dir
    state_gaurd.folder_path = dir;
}

#[tauri::command]
fn get_ports() -> Vec<String> {
    return serial_wrapper::list_ports();
}

#[tauri::command]
fn emit_error(input: String) {
    rfd::MessageDialog::new()
    .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
    .set_title("Port Error")
    .set_description(input.as_str())
    .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
    .show();
    
}

#[tauri::command]
fn send_serial(state: State<AppData>, input: String) {
    let mut state_gaurd = state.0.lock().unwrap();
    let input_format = format!("{}{}", input, state_gaurd.port_items.ending);
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

    // tauri builder
    tauri::Builder::default()
        .manage(AppData(
            // create a new unintlized port
            Mutex::new(Data {
                port: None,
                folder_path: Some(PathBuf::from("/home")),
                port_items: PortItems {
                    port_path: String::from(""),
                    baud_rate: 0,
                    ending: String::from("")
                },
                is_thread_open: Arc::new(AtomicBool::new(true)),
                is_recording: false,
            }),
        ))
        .invoke_handler(tauri::generate_handler![
            set_port_items,
            handle_serial_connect,
            handle_start_record,
            set_folder_path,
            greet,
            get_ports,
            send_serial,
            make_window,
            emit_error
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
