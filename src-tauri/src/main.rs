#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rosc::{decoder, encoder};
use rosc::{OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::Mutex;

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

struct SendConnection(Mutex<OSCSender>);

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
async fn receive() -> (String, String) {
    let socket = UdpSocket::bind("127.0.0.1:9001").expect("couldn't bind to address");
    let mut buf = [0; decoder::MTU];
    let (len, _addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    let filled_buf = &buf[..len];
    let packet = decoder::decode(&filled_buf).expect("failed to decode");
    match packet {
        OscPacket::Message(msg) => {
            let fmt = format!("{:?}", msg.args);
            return (msg.addr, fmt);
        }
        _ => panic!(),
    };
}

const CONFIG_FILE_NAME: &str = "avatarconfig.json";

#[tauri::command]
fn read_avatar_config(app: tauri::AppHandle) -> Result<String, String> {
    let resource_dir = app
        .path_resolver()
        .resource_dir()
        .expect("couldn't get resource dir");
    let avatar_config_path = resource_dir.join(CONFIG_FILE_NAME);
    let contents = std::fs::read(&avatar_config_path).map_err(|e| e.to_string())?;
    String::from_utf8(contents).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_avatar_config(config: String, app: tauri::AppHandle) -> Result<(), String> {
    let resource_dir = app
        .path_resolver()
        .resource_dir()
        .expect("couldn't get resource dir");
    let avatar_config_path = resource_dir.join(CONFIG_FILE_NAME);
    std::fs::write(&avatar_config_path, config).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(SendConnection(Mutex::new(OSCSender::new())))
        .invoke_handler(tauri::generate_handler![
            send_osc_message,
            receive,
            read_avatar_config,
            write_avatar_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
