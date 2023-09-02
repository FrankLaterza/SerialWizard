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

            // if recoding stop recording
            if state_gaurd.is_recording {
                // change menu
                menu_handle
                    .clone()
                    .get_item("start")
                    .set_title("Start")
                    .expect("Failed to change menu");
                state_gaurd.is_recording = false;
            }
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

                    // update the menu
                    let main_window = app.get_window("main").unwrap();
                    let menu_handle = main_window.menu_handle();
                    // set the menu
                    menu_handle
                        .get_item("connect")
                        .set_title("Disconnect")
                        .expect("Failed to change menu");
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
                        // create file name
                        let file_path = path.join("hello.txt");

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
                                // update menu
                                let main_window = app.get_window("main").unwrap();
                                let menu_handle = main_window.menu_handle();
                                // set the menu
                                menu_handle
                                    .get_item("start")
                                    .set_title("Stop")
                                    .expect("Failed to change menu");
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

        let main_window = app.get_window("main").unwrap();
        let menu_handle = main_window.menu_handle();
        // set the menu
        menu_handle
            .get_item("start")
            .set_title("Start")
            .expect("Failed to change menu");
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
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error) // Set the message level to indicate an error
                .set_title("File Error")
                .set_description("Set director before creating file.")
                .set_buttons(rfd::MessageButtons::Ok) // Use OkCancel buttons
                .show();
        }
    }
    return true;
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
                set_folder_path(state);
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
