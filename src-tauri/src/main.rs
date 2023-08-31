// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use serialport::{Error, Result, SerialPort};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};
// todo move into wrapper
use rfd::FileDialog;
use std::fs::File;

// todo move into Data struct
pub struct PortItmes {
    port_path: String,
    baud_rate: String,
}

// todo change Data name because its lame
pub struct Data {
    port: Result<Box<dyn SerialPort>>,
    file_path: Option<PathBuf>,
    file: Arc<Mutex<Option<File>>>,
    port_items: PortItmes,
    ending: String,
    serial_thread: Arc<AtomicBool>,
    record_toggle: Arc<AtomicBool>,
}

pub struct AppData(Mutex<Data>);

fn handle_serial_connect(app: tauri::AppHandle) -> bool {
    // clone the app
    let app_clone = app.clone();

    let state = app_clone.state::<AppData>();
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    // todo check before write
    // get the port items
    let port_path = state_gaurd.port_items.port_path.clone();
    let baud_rate = state_gaurd.port_items.baud_rate.clone();
    let baud_rate_num = baud_rate.parse::<u32>().unwrap();
    let connected = state_gaurd.serial_thread.load(Ordering::Relaxed);

    if connected {
        // anounce killing the thread
        println!("killing thread");
        state_gaurd.serial_thread.store(false, Ordering::Relaxed);

        // set the port as an error
        state_gaurd.port = Err(Error {
            kind: serialport::ErrorKind::Unknown,
            description: String::from(""),
        });

        // update lable menu
        let port_path_clone = state_gaurd.port_items.port_path.clone();
        let baud_rate_clone = state_gaurd.port_items.baud_rate.clone();
        let lable_title: String = format!("Connect: {} | {}", port_path_clone, baud_rate_clone);
        // update the menu
        let main_window = app.get_window("main").unwrap();
        let menu_handle = main_window.menu_handle();
        // set the menu
        menu_handle
            .get_item("connect")
            .set_title(lable_title)
            .expect("Failed to change menu");
        return true;
    } else {
        // start the port
        let port = serial_wrapper::init_port(port_path, baud_rate_num);
        // match on success
        match port {
            Ok(port) => {
                // try clone port
                let port_clone = port.try_clone();
                // set the port
                state_gaurd.port = Ok(port);
                // clone the thread handle (copys a refrence)
                let serial_thread_clone = state_gaurd.serial_thread.clone();
                // enable thread
                serial_thread_clone.store(true, Ordering::Relaxed);
                // get record toggle clone
                let record_toggle_clone = state_gaurd.record_toggle.clone();
                // get the file
                let file_clone = state_gaurd.file.clone();

                // clone app
                let app_clone_thread = app.clone();

                // use clone on thread
                serial_wrapper::start_clone_thread(
                    app_clone_thread,
                    port_clone,
                    serial_thread_clone,
                    record_toggle_clone,
                    file_clone,
                );

                // update the menu
                let main_window = app.get_window("main").unwrap();
                let menu_handle = main_window.menu_handle();
                // set the menu
                menu_handle
                    .get_item("connect")
                    .set_title("Disconnect")
                    .expect("Failed to change menu");

                return true;
            }
            Err(_e) => {
                return false;
            }
        }
    }
}

#[tauri::command]
fn get_ports() -> Vec<String> {
    return serial_wrapper::list_ports();
}

#[tauri::command]
fn send_serial(state: State<AppData>, input: String) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();
    println!("writng string: {}", input);
    let input_format = format!("{}{}", input, state_gaurd.ending);
    let write = serial_wrapper::write_serial(&mut state_gaurd.port, input_format.as_str());
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

fn handle_start_record(app: tauri::AppHandle) -> bool {
    // get menu
    let main_window = app.clone().get_window("main").unwrap();
    let menu_handle = main_window.menu_handle();
    // get state
    let state = app.state::<AppData>();
    let mut state_gaurd = state.0.lock().unwrap();

    // check state of recroding
    match state_gaurd.record_toggle.load(Ordering::Relaxed) {
        // if recording prompt to stop
        true => {
            // stop recording
            println!("Stop recording");
            state_gaurd.record_toggle.store(false, Ordering::Relaxed);
            state_gaurd.file = Arc::new(Mutex::new(None.into()));
            // set file to none

            // update menu to next state
            menu_handle
                .get_item("start")
                .set_title("Start")
                .expect("Failed to change menu");
        }
        false => {
            match &state_gaurd.file_path {
                Some(path) => {
                    println!("Start recording");
                    let file_path = path.join("SerialLog.txt");

                    state_gaurd.file = Arc::new(Mutex::new(
                        Some(File::create(file_path).expect("Could not create file")).into(),
                    ));
                    // start recoding on the file
                    state_gaurd.record_toggle.store(true, Ordering::Relaxed);
                }
                None => {
                    println!("File path not set");
                    state_gaurd.record_toggle.store(false, Ordering::Relaxed);
                }
            }

            menu_handle
                .get_item("start")
                .set_title("Stop")
                .expect("Failed to change menu");
        }
    }

    // set menu stop
    return true;
    // destroy the port by opending to nothing
}

fn set_ending(app: tauri::AppHandle, ending: String) {
    let state = app.state::<AppData>();
    let mut state_gaurd = state.0.lock().unwrap();
    // todo show input to user somehow
    println!("{}", ending);

    if ending == "\\n\\r" {
        let combo = format!("{}{}", '\n', '\r');
        state_gaurd.ending = combo;
    } else if ending == "\\n" {
        state_gaurd.ending = String::from('\n');
    } else if ending == "\\r" {
        state_gaurd.ending = String::from('\r');
    } else {
        state_gaurd.ending = String::from("");
    }
}

fn set_port(app: tauri::AppHandle, port_path: String) {
    let state = app.state::<AppData>();
    let mut state_gaurd = state.0.lock().unwrap();
    state_gaurd.port_items.port_path = port_path;

    // update lable menu
    let port_path_clone = state_gaurd.port_items.port_path.clone();
    let baud_rate_clone = state_gaurd.port_items.baud_rate.clone();
    let lable_title: String = format!("Connect: {} | {}", port_path_clone, baud_rate_clone);
    // update the menu
    let main_window = app.get_window("main").unwrap();
    let menu_handle = main_window.menu_handle();
    // set the menu
    menu_handle
        .get_item("connect")
        .set_title(lable_title)
        .expect("Failed to change menu");
}

fn set_baud(app: tauri::AppHandle, baud_rate: String) {
    let state = app.state::<AppData>();
    let mut state_gaurd = state.0.lock().unwrap();
    state_gaurd.port_items.baud_rate = baud_rate;

    // update lable menu
    let port_path_clone = state_gaurd.port_items.port_path.clone();
    let baud_rate_clone = state_gaurd.port_items.baud_rate.clone();
    let lable_title: String = format!("Connect: {} | {}", port_path_clone, baud_rate_clone);
    // update the menu
    let main_window = app.get_window("main").unwrap();
    let menu_handle = main_window.menu_handle();
    // set the menu
    menu_handle
        .get_item("connect")
        .set_title(lable_title)
        .expect("Failed to change menu");
}

fn create_ending_items(endings: Vec<&str>) -> Menu {
    let mut menu = Menu::new();
    for end in endings {
        menu = menu.add_item(CustomMenuItem::new(end, end));
    }

    return menu;
}

fn create_port_items() -> Menu {
    let mut menu = Menu::new();
    let ports: Vec<String> = get_ports();
    for port in ports {
        menu = menu.add_item(CustomMenuItem::new(port.clone(), port));
    }
    // todo add refresh button
    // menu = menu.add_item(CustomMenuItem::new("refresh", "Refresh"));

    return menu;
}

// todo find better solution
// fn refresh_port_items(app: tauri::AppHandle) {
//     // get the existing menu
//     let main_window = app.get_window("main").unwrap();
//     let menu_handle = main_window.menu_handle();

//     let ports: Vec<String> = get_ports();
//     for port in ports {
//         // custom tauri function
//         // menu_handle.print_ids();
//     }
// }

fn create_baud_items(baud_rates: Vec<&str>) -> Menu {
    let mut menu = Menu::new();
    for baud in baud_rates {
        menu = menu.add_item(CustomMenuItem::new(baud, baud));
    }

    return menu;
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
                port: Err(Error {
                    kind: serialport::ErrorKind::Unknown,
                    description: String::from(""),
                }),
                file_path: Some(PathBuf::from("/home")),
                file: Arc::new(Mutex::new(None)),
                port_items: PortItmes {
                    port_path: String::from(""),
                    baud_rate: String::from("9800"),
                },
                ending: String::from(""),
                record_toggle: Arc::new(AtomicBool::new(false)),
                serial_thread: Arc::new(AtomicBool::new(false)),
            }),
        ))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_ports,
            send_serial,
            make_window,
        ])
        .menu(
            Menu::new()
                .add_submenu(Submenu::new(
                    "Record",
                    Menu::new()
                        .add_item(CustomMenuItem::new("set_directory", "Set Directory"))
                        .add_item(CustomMenuItem::new("start", "Start")),
                ))
                .add_submenu(Submenu::new(
                    "Serial",
                    Menu::new()
                        // todo add changing menu buttons
                        .add_item(CustomMenuItem::new("connect", "Connect: None"))
                        .add_submenu(Submenu::new("Ending", create_ending_items(endings.clone())))
                        .add_submenu(Submenu::new("Ports", create_port_items()))
                        .add_submenu(Submenu::new("Bauds", create_baud_items(baud_rates.clone()))),
                )),
        )
        .on_menu_event(move |event| match event.menu_item_id() {
            "set_directory" => {
                let app = event.window().app_handle();
                let state = app.state::<AppData>();
                // handle error
                if !set_folder_path(state) {
                    todo!();
                };
            }
            "start" => {
                let app = event.window().app_handle();
                handle_start_record(app);
            }
            "connect" => {
                let app = event.window().app_handle();
                handle_serial_connect(app);
            }
            "refresh" => {
                // let app = event.window().app_handle();
                // refresh_port_items(app);
            }
            _ => {
                for end in &endings {
                    if end == &event.menu_item_id() {
                        let app = event.window().app_handle();
                        set_ending(app, end.to_string());
                    }
                }

                for baud in &baud_rates {
                    if baud == &event.menu_item_id() {
                        let app = event.window().app_handle();
                        set_baud(app, baud.to_string());
                    }
                }

                // get the ports from the event
                let ports = get_ports();
                for port in ports {
                    if port == event.menu_item_id() {
                        let app = event.window().app_handle();
                        set_port(app, port);
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
