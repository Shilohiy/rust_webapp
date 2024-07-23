use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    println!("Connected to the server!");
    let message = "Hello from the client!";
    stream.write(message.as_bytes()).unwrap();

    let mut buffer = vec![0; message.len()];
    stream.read(&mut buffer).unwrap();

    println!(  
        "Received from server: {:?}",
         str::from_utf8(&buffer).unwrap()
    );
}