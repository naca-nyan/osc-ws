#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{UdpSocket};

#[tauri::command]
fn send_osc_message(addr: String, value: String, typ: String) {
  println!("{} {} {}", addr, value, typ);
  let localhost = "127.0.0.1:0";
  let vrc = "127.0.0.1:9000";
  let sock = UdpSocket::bind(localhost).expect("cannot bind localhost");
  sock.connect(vrc).expect("connection failed");
  let value = value.parse().unwrap();
  let buf = encoder::encode(&OscPacket::Message(OscMessage {
    addr: addr,
    args: vec![OscType::Int(value)],
  })).expect("cannot encode");
  sock.send(&buf);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler!(send_osc_message))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
