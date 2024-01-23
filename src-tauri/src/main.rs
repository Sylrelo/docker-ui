// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread::sleep,
    time::Duration,
};

use serde::Serialize;
use tauri::Manager;

#[derive(Serialize)]
struct Response {
    code: i32,
    data: String,
}

#[tauri::command]
async fn socket_communication(method: &str, endpoint: &str) -> Result<Response, ()> {
    let mut sock = match UnixStream::connect("/var/run/docker.sock") {
        Ok(l) => l,
        Err(err) => {
            println!("Error: {}", err);
            return Err(());
        }
    };

    let _ = sock.set_write_timeout(None);
    let _ = sock.set_read_timeout(None);
    // let _ = sock.set_nonblocking(true);

    let _ = sock.write_all(method.as_bytes());
    let _ = sock.write_all(b" ");
    let _ = sock.write_all(endpoint.as_bytes());
    let _ = sock.write_all(
        b" HTTP/1.1\r\nHost: localhost\r\nUser-Agent: dockermonitor/1.0.0\r\nAccept: */*\r\n",
    );

    if method.eq("POST") {
        // let _ = sock.write_all(b"Content-Length: 0\r\n");
        let _ = sock.write_all(b"Content-Type: application/json\r\n");
    }

    let _ = sock.write_all(b"\r\n");

    if method.eq("POST") {}

    sleep(Duration::from_millis(500));
    let _ = sock.shutdown(std::net::Shutdown::Write);

    let mut response_buffer = String::new();
    let _ = sock.read_to_string(&mut response_buffer);
    let _ = sock.shutdown(std::net::Shutdown::Both);

    let mut response_end_index = response_buffer.len();
    let mut response_start_index = match response_buffer.find("\r\n\r\n") {
        Some(i) => i,
        None => {
            println!("None !");
            return Err(());
        }
    };

    // println!("{:?}", response_start_index);
    if response_buffer.contains("Transfer-Encoding: chunked") {
        response_start_index = match response_buffer
            .get(response_start_index + 4..)
            .unwrap_or_default()
            .find("\r\n")
        {
            Some(i) => {
                // println!("{} {}", i, response_start_index);
                response_start_index + i + 4 + 2
            }
            None => {
                println!("None !");
                return Err(());
            }
        };
        // println!("{:?}", response_start_index);

        response_end_index = response_end_index - 8;
        // println!("new pos {}", response_start_index);
        //    let  response_buffer.repl
    }
    // println!(
    //     "====================\n{:?}====================\n",
    //     response_buffer
    // );

    let response = response_buffer
        .get(response_start_index..response_end_index)
        .unwrap_or_default()
        .trim_end();

    if !endpoint.contains("containers/json") {
        println!("{} {}", method, endpoint);
        println!("=> {}", response_buffer);
    }

    return Ok(Response {
        code: 0,
        data: response.to_owned(),
    });
}

fn main() {
    // socket_communication("GET", "http://localhost/version");

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            // window.close_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![socket_communication])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
