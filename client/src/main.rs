use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server");

    let message = "Hello, server!\n";  // Adding a newline as the message delimiter
    stream.write_all(message.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("Received response from server: {}", response);
}
