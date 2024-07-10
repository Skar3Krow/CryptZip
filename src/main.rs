use std::{env, fs, io::{Read, Result, Write}, net::{TcpListener, TcpStream}, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();    
    for stream in listener.incoming() {
        match stream {
            Ok( _stream) => {
                println!("accepted new connection");
                thread::spawn(|| handle_client(_stream).unwrap());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<()>{
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    let request=String::from_utf8_lossy(&buf[..]);
    let lines: Vec<&str> = request.split("\r\n").collect();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
            }else if tokens[1].starts_with("/files/"){
                let filename = tokens[1].replace("/files/", "");
                let env_args: Vec<_> = env::args().collect();
                let mut filepath = env_args[2].clone();
                filepath.push('/');
                filepath.push_str(&filename);
                let file = fs::read(&filepath);
                let mut response = String::from("");
                match file {
                    Ok(f) => {
                        response=format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", f.len(), String::from_utf8(f).expect("File content"));
                    },
                    Err(_) => {
                        response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
                        
                    }
                }
                stream.write(response.as_bytes());
            }else if tokens[1].starts_with("/user-agent") {
                let response = lines[2].replace("User-Agent: ", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            }else if tokens[1].starts_with("/echo/") {
                let response = tokens[1].replace("/echo/", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            }else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        },
        _ => println!("Method not Found: {:?}", tokens[0])

    }
    Ok(())
    
}