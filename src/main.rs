use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();    
    for stream in listener.incoming() {
        match stream {
            Ok(ref _stream) => {
                println!("accepted new connection");
                handle_client(stream.expect("Failed to handle client request"));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return;
        }
        if bytes_read > 40 {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").expect("Failed to write to client");
        } else {
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").expect("Failed to write to client");
        }
    }
}