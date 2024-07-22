use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use chrono::Utc;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4096*10*3];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("\n讀取 socket 失敗：{}\n", e);
            return;
        }
    };
    if bytes_read == 0 { return; } // connection closed

    let request = match std::str::from_utf8(&buffer[0..bytes_read]) {
        Ok(v) => v.to_string(),
        Err(e) => {
            println!("\n無效的 UTF-8 序列：{}\n", e);
            return;
        },
    };
    println!("\nHTTP request：\n\n{}\n\n[C]======>[S]\n", request);

    let response_body = "<!DOCTYPE html>\n<html>\n<head>\n  <title>An Example Page</title>\n</head>\n<body>\n  Hello, world!\n</body>\n</html>";
    let current_time = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
    let response_header = format!("HTTP/1.1 200 OK\nDate: {}\nServer: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)\nLast-Modified: Wed, 08 Jan 2003 23:11:55 GMT\nETag: \"3f80f-1b6-3e1cb03b\"\nContent-Type: text/html; charset=UTF-8\nContent-Length: {}\nAccept-Ranges: bytes\nConnection: close", current_time, response_body.chars().count());
    let response = format!("{}\n\r\n{}", response_header, response_body);
    let res_bytes = response.as_bytes();

    println!("\nHTTP response：\n\n{}\n\n[C]<=======[S]\n", response);
    println!("\n編碼後的 byte array：\n{:?}\n\n*========*\n", res_bytes);

    if let Err(e) = stream.write_all(&res_bytes) {
        eprintln!("\n寫入 socket 失敗：{}\n", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("\n刷新 socket 失敗：{}\n", e);
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| { handle_client(stream); });
            }
            Err(e) => { eprintln!("\n無法連接：{}\n", e); }
        }
    }
}
