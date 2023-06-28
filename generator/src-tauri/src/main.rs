// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::prelude::*;
use std::net::TcpStream;
use tauri::{command, Manager};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_tcp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn send_tcp(ip: String, port: u16, content: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect((ip, port))?;
    stream.write_all(content.as_bytes())?;
    let mut buffer = [0; 1024]; 
    let size = stream.read(&mut buffer).await?;  // 使用 async read
    let response = String::from_utf8_lossy(&buffer[..size]).to_string();
    Ok(response)
}
