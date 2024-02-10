use std::{fs};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn get_file_path (filename: &str) -> String {
    let http_dir = "src".to_string() + "/pages";
    let file_path = http_dir + "/" + filename;
    file_path
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")

    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")

    };

    let content = fs::read_to_string(get_file_path(filename)).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);
    // let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
