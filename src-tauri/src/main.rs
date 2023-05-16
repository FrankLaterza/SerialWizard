// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod serial_wrapper;
use serialport::{ Error, Result, SerialPort };
use std::sync::Mutex;
use tauri::State;

pub struct Data {
    port_name: String,
    baud_rate: u32,
    port: Result<Box<dyn SerialPort>>,
}

pub struct AppData(Mutex<Data>);

fn main() {
    #[tauri::command]
    fn open_serial(state: State<AppData>) -> bool {
        let mut state_gaurd = state.0.lock().unwrap();
        println!("{}", state_gaurd.port_name);

        state_gaurd.port = serial_wrapper::init_port(&state_gaurd.port_name, state_gaurd.baud_rate);

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
    fn set_baud(state: State<AppData>, board_rate: u32) {
        let mut state_gaurd = state.0.lock().unwrap();
        state_gaurd.baud_rate = board_rate;
        println!("{}", state_gaurd.baud_rate);
    }

    #[tauri::command]
    fn set_port(state: State<AppData>, port_name: String) {
        let mut state_gaurd = state.0.lock().unwrap();
        state_gaurd.port_name = port_name;
        println!("{}", state_gaurd.port_name);
    }

    #[tauri::command]
    fn get_ports() -> Vec<String> {
        return serial_wrapper::list_ports();
    }

    #[tauri::command]
    fn send_serial(state: State<AppData>, mut input: String) -> bool {
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
    fn receive_update(state: State<AppData>) -> String {
        // return String::from("hello world");
        let mut state_gaurd = state.0.lock().unwrap();

        match serial_wrapper::receive_serial(&mut state_gaurd.port) {
            Ok(p) => {
                println!("found serial: {}", p);
                return p;
            }
            Err(_) => {
                return String::from("");
            }
        }
    }

    // receive_update(state);

    #[tauri::command]
    fn receive_serial(state: State<AppData>) -> String {
        let mut state_gaurd = state.0.lock().unwrap();

        match serial_wrapper::wait_receive_serial(&mut state_gaurd.port) {
            Ok(s) => {
                println!("read found!");
                return s;
            }
            Err(e) => {
                println!("an error has occured trying to read: {}", e);
                return String::from("");
            }
        }
    }

    #[tauri::command]
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    // make a new window
    #[tauri::command]
    async fn make_window(handle: tauri::AppHandle) {
        tauri::WindowBuilder
            ::new(&handle, "Setup", tauri::WindowUrl::App("/about".into()))
            .inner_size(500.0, 500.0)
            .resizable(false)
            .always_on_top(true)
            .title("Setup")
            .build()
            .unwrap();
    }

    tauri::Builder
        ::default()
        .manage(
            AppData(
                Mutex::new(Data {
                    port_name: String::from(""),
                    baud_rate: 115200,
                    port: Err(Error {
                        kind: serialport::ErrorKind::Unknown,
                        description: String::from(""),
                    }),
                })
            )
        )
        .invoke_handler(
            tauri::generate_handler![
                greet,
                open_serial,
                set_baud,
                set_port,
                get_ports,
                send_serial,
                receive_update,
                receive_serial,
                make_window,
                close_port
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}