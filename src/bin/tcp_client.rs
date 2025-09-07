// Problem statement: Estabilish TCP client-server Communication
// create a TCP server that listens on 4040  and echoes back messages to client side

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // creating 512 bytes each 0
    // buffer is mutable,
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
             break; 
            }
        stream.write_all(&buffer[..bytes_read]).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();
    let port  = 4040;
    println!("Server listening on {}",port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
