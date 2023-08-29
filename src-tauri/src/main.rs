// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use serialport::{Error, Result, SerialPort};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, State, Submenu, WindowBuilder};

// todo move into wrapper
use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

pub struct Data {
    port: Result<Box<dyn SerialPort>>,
    file_path: Option<PathBuf>,
}

pub struct AppData(Mutex<Data>);

#[tauri::command]
fn open_serial(
    state: State<AppData>,
    app: tauri::AppHandle,
    port_name: String,
    baud_rate: u32,
) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();

    state_gaurd.port = serial_wrapper::init_port(app, &port_name, baud_rate);

    let port = &state_gaurd.port;
    match port {
        Ok(_port) => {
            return true;
        }
        Err(_e) => {
            return false;
        }
    }
}

#[tauri::command]
fn close_port(state: State<AppData>) {
    let state_guard = state.0.lock().unwrap();
    // ... use the state_guard as needed
    std::mem::drop(state_guard); // calls the Drop implementation of Data
}

#[tauri::command]
fn get_ports() -> Vec<String> {
    return serial_wrapper::list_ports();
}

#[tauri::command]
fn send_serial(state: State<AppData>, input: String) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();
    // input.push('\r');
    println!("writng string: {}", input);
    let write = serial_wrapper::write_serial(&mut state_gaurd.port, input.as_str());
    match write {
        Ok(_) => {
            println!("write successful");
            return true;
        }
        Err(e) => {
            println!("an error has occured write to read: {}", e);
            return false;
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

#[tauri::command]
fn set_folder_path(state: State<AppData>) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();

    let dir = FileDialog::new().set_directory("/").pick_folder();
    // print the path
    match dir {
        Some(path) => {
            // add the file name to the path get string. todo fix
            let file_path = path.join("");
            println!("path set {}", file_path.to_string_lossy());
            // set path
            state_gaurd.file_path = Some(file_path);
            // save to path
        }
        None => {
            return false;
        }
    }
    return true;
}

#[tauri::command]
fn start_record(state: State<AppData>) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();
    return true;
    // destroy the port by opending to nothing
}

#[tauri::command]
fn stop_record(state: State<AppData>) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();

    let dir = FileDialog::new().set_directory("/").pick_folder();
    // print the path
    match dir {
        Some(path) => {
            // add the file name to the path
            let file_path = path.join("hello.txt"); // Use the selected path to create a file path
                                                    // print path
            println!("path set {}", file_path.to_string_lossy());
            // set path
            state_gaurd.file_path = Some(file_path);
            // save to path
        }
        None => {
            return false;
        }
    }
    return true;
}

fn create_port_items() -> Menu {
    let ports: Vec<String> = get_ports();
    let mut menu = Menu::new();

    for port in ports {
        menu = menu.add_item(CustomMenuItem::new(port.clone(), port));
    }

    return menu;
}

fn create_baud_items() -> Menu {
    let baud_rates: Vec<&str> = vec![
        "300", "1200", "2400", "4800", "9600", "19200", "38400", "57600", "74880", "115200",
        "230400", "250000", "500000", "1000000", "2000000",
    ];

    let mut menu = Menu::new();

    for baud in baud_rates {
        menu = menu.add_item(CustomMenuItem::new(baud.clone(), baud));
    }

    return menu;
}

#[tauri::command]
fn print_file_path(state: State<AppData>){
    let state_gaurd = state.0.lock().unwrap();
    let path = state_gaurd.file_path.as_ref().expect("path not set");
    println!("{}",path.to_string_lossy());
}

fn main() {
    // tauri builder
    tauri::Builder::default()
        .manage(AppData(
            // create a new unintlized port
            Mutex::new(Data {
                port: Err(Error {
                    kind: serialport::ErrorKind::Unknown,
                    description: String::from(""),
                }),
                file_path: Some(PathBuf::from("/dir")),
            }),
        ))
        .invoke_handler(tauri::generate_handler![
            greet,
            open_serial,
            get_ports,
            send_serial,
            make_window,
            close_port,
            set_folder_path,
            print_file_path
        ])
        .menu(
            Menu::new()
                //.add_item(CustomMenuItem::new("file", "File"))
                .add_submenu(Submenu::new(
                    "File",
                    Menu::new().add_item(CustomMenuItem::new("open", "Open")),
                ))
                .add_submenu(Submenu::new(
                    "Tools",
                    Menu::new()
                        .add_submenu(Submenu::new("Ports", create_port_items()))
                        .add_submenu(Submenu::new("Bauds", create_baud_items())),
                )),
        )
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => {
                let app = event.window().app_handle();
                let state = app.state::<State<AppData>>();
                let mut gaurd = state.0.lock().unwrap();
                gaurd.file_path = Some(PathBuf::from("/dir"));
            }
            "close" => {

            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
