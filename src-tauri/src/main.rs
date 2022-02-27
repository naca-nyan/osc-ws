#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rosc::{decoder, encoder};
use rosc::{OscMessage, OscPacket, OscType};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;

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
    }))
    .expect("cannot encode");
    let _res = sock.send(&buf).expect("cannot send");
}

fn reciever() {
    let mut args = HashMap::new();
    let socket = UdpSocket::bind("127.0.0.1:9001").expect("couldn't bind to address");
    let mut buf = [0; decoder::MTU];
    loop {
        println!("-----------");
        let (len, _addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf = &buf[..len];
        let packet = decoder::decode(&filled_buf).expect("failed to decode");
        match packet {
            OscPacket::Message(msg) => {
                args.insert(msg.addr, msg.args);
            }
            _ => {}
        }
        for (k, v) in args.iter() {
            println!("{}: {:?}", k, v)
        }
    }
}

fn main() {
    thread::spawn(reciever);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler!(send_osc_message))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
