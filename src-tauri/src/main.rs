// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use serialport::{Error, Result, SerialPort};
use std::fmt::format;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::sync::{Mutex, MutexGuard};
use tauri::{App, CustomMenuItem, Manager, Menu, MenuItem, State, Submenu, WindowBuilder};
// todo move into wrapper
use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

// todo move into Data struct
pub struct PortItmes {
    port_path: String,
    baud_rate: String,
}

// todo change Data name because its lame
// todo group data into sub structs or impl.
pub struct Data {
    // todo change to option
    port: Result<Box<dyn SerialPort>>,
    file_path: Option<PathBuf>,
    port_items: PortItmes,
    ending: String,
    is_thread_open: Arc<AtomicBool>,
    // todo track currect menu itmes
}

pub struct AppData(Mutex<Data>);

fn handle_serial_connect(app: tauri::AppHandle) -> bool {
    // clone the app
    let app_clone = app.clone();

    let state = app_clone.state::<AppData>();
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    // clone state gaurd data
    let port_path = state_gaurd.port_items.port_path.clone();
    let baud_rate = state_gaurd.port_items.baud_rate.clone();

    // todo check before write
    // get the port items
    let is_thread_open = state_gaurd.is_thread_open.load(Ordering::Relaxed);

    if (is_thread_open) {

        // anounce killing the thread
        println!("killing thread");
        // kill thread. todo confim thead kill
        state_gaurd.is_thread_open.store(false, Ordering::Relaxed);

        // set the port as an error
        state_gaurd.port = Err(Error {
            kind: serialport::ErrorKind::Unknown,
            description: String::from(""),
        });

        // update lable menu
        let lable_title: String = format!("Connect: {} | {}", port_path, baud_rate);
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
                // convert baud to num
                let baud_rate_num = baud_rate.parse::<u32>().unwrap();
                // try clone port
                let port_clone = port.try_clone().unwrap();
                // set the port
                state_gaurd.port = Ok(port);
                // clone the thread handle (copys a refrence)
                let is_thread_opened = state_gaurd.is_thread_open.clone();
                // enable thread
                is_thread_opened.store(true, Ordering::Relaxed);
                // use clone on thread
                serial_wrapper::start_clone_thread(
                    app.clone(),
                    port_clone,
                    is_thread_opened,
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
                // todo make menu show error
                // port could not be created

                // couldn't open make thread false 
                state_gaurd.is_thread_opened.store(true, Ordering::Relaxed);
                return false;
            }
        }
    }
}

// kills current thread and starts recording
fn handle_start_record(app: tauri::AppHandle) -> bool {
    // clone the app
    let app_clone = app.clone();

    let state = app_clone.state::<AppData>();
    // unclock gaurd
    let mut state_gaurd = state.0.lock().unwrap();

    // clone state gaurd data
    let port_path = state_gaurd.port_items.port_path.clone();
    let baud_rate = state_gaurd.port_items.baud_rate.clone();

    // todo check before write
    // get the port items
    let is_thread_open = state_gaurd.is_thread_open.load(Ordering::Relaxed);

    // if there is a thread open (must be a valid port)
    if (is_thread_open) {
        // anounce killing the thread
        println!("killing thread");
        // kill thread. todo confim thead kill
        state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
        // get the opened port
        let port_clone = state_gaurd.port.unwrap();
        // open thread
        state_gaurd.is_thread_open.store(false, Ordering::Relaxed);
        // start serial on port
        serial_wrapper::start_record_on_port(
            app.clone(),
            port_clone,
            is_thread_opened,
        );


    } else {
        // announce error
        // must be connected to serial port to start recording
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

fn start_record(state: State<AppData>) -> bool {
    let mut state_gaurd = state.0.lock().unwrap();
    return true;
    // destroy the port by opending to nothing
}

fn save_record(state: State<AppData>) -> bool {
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

fn set_ending(app: tauri::AppHandle, ending: String) {
    let state = app.state::<AppData>();
    let mut state_gaurd = state.0.lock().unwrap();
    // todo show input to user somehow
    println!("{}", ending);

    if (ending == "\\n\\r") {
        let combo = format!("{}{}", '\n', '\r');
        state_gaurd.ending = combo;
    } else if (ending == "\\n") {
        state_gaurd.ending = String::from('\n');
    } else if (ending == "\\r") {
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
    // add refresh button
    menu = menu.add_item(CustomMenuItem::new("refresh", "Refresh"));

    return menu;
}

// todo find better solution
fn refresh_port_items(app: tauri::AppHandle){

    // get the existing menu
    let main_window = app.get_window("main").unwrap();
    let menu_handle = main_window.menu_handle();
    
    let ports: Vec<String> = get_ports();
    for port in ports {
        // custom tauri function
        // menu_handle.print_ids();
    }

}

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
                port_items: PortItmes {
                    port_path: String::from(""),
                    baud_rate: String::from(""),
                },
                ending: String::from(""),
                is_thread_open: Arc::new(AtomicBool::new(false)),
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
                        .add_item(CustomMenuItem::new("start", "Start"))
                        .add_item(CustomMenuItem::new("save", "Save")),
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
                if (!set_folder_path(state)) {
                    todo!();
                };
            }
            "start" => {
                let app = event.window().app_handle();
                let state = app.state::<AppData>();
                start_record(state);
            }
            "save" => {
                let app = event.window().app_handle();
                let state = app.state::<AppData>();
                save_record(state);
            }
            "connect" => {
                let app = event.window().app_handle();
                handle_serial_connect(app);
            }
            "refresh" => {
                let app = event.window().app_handle();
                refresh_port_items(app);
            }
            _ => {
                for end in &endings {
                    if (end == &event.menu_item_id()) {
                        let app = event.window().app_handle();
                        set_ending(app, end.to_string());
                    }
                }

                for baud in &baud_rates {
                    if (baud == &event.menu_item_id()) {
                        let app = event.window().app_handle();
                        set_baud(app, baud.to_string());
                    }
                }

                // get the ports from the event
                let ports = get_ports();
                for port in ports {
                    if (port == event.menu_item_id()) {
                        let app = event.window().app_handle();
                        set_port(app, port);
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
