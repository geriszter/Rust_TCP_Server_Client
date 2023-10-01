use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;


fn handle_client(mut stream: TcpStream) {
    let mut buf = Vec::new();
    loop {
        let mut chunk = vec![0; 1024];
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                if buf.iter().any(|&x| x == b'\n') {
                    let received = String::from_utf8_lossy(&buf);
                    println!("Received message from client: {}", received);
                    break;
                }
            }
            Err(_) => {
                println!("Failed to read from socket");
                break;
            }
        }
    }

    stream.write_all(b"Message received by the server").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
}
