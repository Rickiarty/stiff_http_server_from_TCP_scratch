use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use chrono::Utc;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4096*3];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("讀取 socket 失敗：{}", e);
            return;
        }
    };
    if bytes_read == 0 { return; } // connection closed

    let request = match std::str::from_utf8(&buffer[0..bytes_read]) {
        Ok(v) => v.to_string(),
        Err(e) => {
            println!("無效的 UTF-8 序列：{}", e);
            return;
        },
    };
    println!("{}\n\n======\n", request);

    let response_body = "<!DOCTYPE html>\n<html>\n<head>\n  <title>An Example Page</title>\n</head>\n<body>\n  Hello World.\n</body>\n</html>";
    let current_time = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
    let response_header = format!("HTTP/1.1 200 OK\nDate: {}\nServer: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)\nLast-Modified: Wed, 08 Jan 2003 23:11:55 GMT\nETag: \"3f80f-1b6-3e1cb03b\"\nContent-Type: text/html; charset=UTF-8\nContent-Length: {}\nAccept-Ranges: bytes\nConnection: close\n", current_time, response_body.chars().count());
    let response = format!("{}\r\n{}", response_header, response_body);
    let res_bytes = response.as_bytes();

    println!("字串：{}", response);
    println!("編碼後的 byte array：{:?}", res_bytes);

    if let Err(e) = stream.write_all(&res_bytes) {
        eprintln!("寫入 socket 失敗：{}", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("刷新 socket 失敗：{}", e);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| { handle_client(stream); });
            }
            Err(e) => { eprintln!("無法連接：{}", e); }
        }
    }
}
