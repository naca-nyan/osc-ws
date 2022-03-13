#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rosc::{decoder, encoder};
use rosc::{OscMessage, OscPacket, OscType};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

use tauri::State;

struct OSCSender {
    sock: UdpSocket,
}

impl OSCSender {
    fn new() -> Self {
        let localhost = "127.0.0.1:0";
        let vrc = "127.0.0.1:9000";
        let sock = UdpSocket::bind(localhost).expect("cannot bind localhost");
        sock.connect(vrc).expect("connection failed");
        OSCSender { sock }
    }
    fn send(&self, packet: &OscPacket) -> Result<usize, std::io::Error> {
        let buf = encoder::encode(packet).expect("cannot encode");
        let res = self.sock.send(&buf);
        res
    }
}

struct OSCReceiver {
    states: Arc<Mutex<HashMap<String, Vec<OscType>>>>,
}

impl OSCReceiver {
    fn new() -> Self {
        let vrc = "127.0.0.1:9001";
        let sock = UdpSocket::bind(vrc).expect("couldn't bind to address");
        let states = Arc::new(Mutex::new(HashMap::new()));
        let states0 = Arc::clone(&states);
        let receive_forever = move || loop {
            let mut buf = [0; decoder::MTU];
            let (len, _addr) = sock.recv_from(&mut buf).expect("Didn't receive data");
            let filled_buf = &buf[..len];
            let packet = decoder::decode(&filled_buf).expect("failed to decode");
            match packet {
                OscPacket::Message(msg) => {
                    let args = msg.args;
                    states0.lock().unwrap().insert(msg.addr, args);
                }
                _ => (),
            };
        };
        thread::spawn(receive_forever);
        OSCReceiver { states }
    }
}

struct SendConnection(Mutex<OSCSender>);
struct ReceiveConnection(Mutex<OSCReceiver>);

#[tauri::command]
fn send_osc_message(
    addr: String,
    value: String,
    typ: String,
    connection: State<'_, SendConnection>,
) {
    println!("{} {} {}", addr, value, typ);
    let arg = match typ.as_str() {
        "Int" => {
            let value = value.parse().unwrap();
            OscType::Int(value)
        }
        "Bool" => {
            let value = value.parse().unwrap();
            OscType::Bool(value)
        }
        "Float" => {
            let value = value.parse().unwrap();
            OscType::Float(value)
        }
        _ => panic!("Type not implemented"),
    };
    let packet = OscPacket::Message(OscMessage {
        addr: addr,
        args: vec![arg],
    });
    connection
        .0
        .lock()
        .unwrap()
        .send(&packet)
        .expect("cannot send");
}

#[tauri::command]
fn get_state(key: String, connection: State<'_, ReceiveConnection>) -> Option<String> {
    connection
        .0
        .lock()
        .unwrap()
        .states
        .lock()
        .unwrap()
        .get(&key)
        .map(|x| format!("{:?}", x))
}

#[tauri::command]
fn get_states(connection: State<'_, ReceiveConnection>) -> Vec<(String, String)> {
    connection
        .0
        .lock()
        .unwrap()
        .states
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.into(), format!("{:?}", v)))
        .collect()
}

fn get_avatar_config_path<'a>(app: tauri::AppHandle) -> std::path::PathBuf {
    let filename = "avatarconfig.json";
    let resource_dir = app
        .path_resolver()
        .resource_dir()
        .expect("couldn't get resource dir");
    let avatar_config_path = resource_dir.join(filename);
    avatar_config_path
}

#[tauri::command]
fn load_avatar_config(app: tauri::AppHandle) -> Result<String, String> {
    let path = get_avatar_config_path(app);
    let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(contents)
}

#[tauri::command]
fn save_avatar_config(config: String, app: tauri::AppHandle) -> Result<(), String> {
    let path = get_avatar_config_path(app);
    std::fs::write(&path, config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn read_avatar_config(_avatarid: String) -> Result<String, String> {
    unimplemented!()
}
fn main() {
    tauri::Builder::default()
        .manage(SendConnection(Mutex::new(OSCSender::new())))
        .manage(ReceiveConnection(Mutex::new(OSCReceiver::new())))
        .invoke_handler(tauri::generate_handler![
            send_osc_message,
            get_states,
            get_state,
            read_avatar_config,
            load_avatar_config,
            save_avatar_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
