// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::all::GBK;
use encoding::{Encoding};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_tcp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn send_tcp(ip: String, port: i32, content: &str, charset:&str) -> String {
    // format!("ip: {}, port: {}, message: {}", ip, port, content)
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
    // 根据传入的字符集转换为字节流
    if charset == "UTF-8" {
        stream.write(content.as_bytes()).unwrap();
    } else {
        let gbk = GBK.encode(content, encoding::EncoderTrap::Ignore).unwrap();
        stream.write(&gbk).unwrap();
    }
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // let response = String::from_utf8_lossy(&buffer[..]);
    if charset == "GBK" {
        let response = GBK.decode(&buffer, encoding::DecoderTrap::Ignore).unwrap();
        return response;
    }
    let response = String::from_utf8_lossy(&buffer[..]);
    response.to_string()
}
