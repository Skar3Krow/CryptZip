use std::{env, fs::{self, File}, io::{Read, Result, Write}, net::{TcpListener, TcpStream}, thread};
use flate2::{write::GzEncoder, Compression};


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
                let response;
                match file {
                    Ok(f) => {
                        response=format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", f.len(), String::from_utf8(f).expect("File content"));
                    },
                    Err(_) => {
                        response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
                        
                    }
                }
                stream.write(response.as_bytes())?;
            }else if tokens[1].starts_with("/user-agent") {
                let response = lines[2].replace("User-Agent: ", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            }else if tokens[1].starts_with("/echo/") {
                let response = tokens[1].replace("/echo/", "");
                if lines.len() > 4 {
                    let mut checker=false;
                    let invalidator : Vec<&str> = lines[2].split(" ").collect();
                    for x in invalidator.iter() {
                        if x.starts_with("gzip") {
                            checker=true;
                        }
                    }
                    if checker==true {
                        let (compressed, len) = get_gzip(response.to_owned());
                        stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n", len).as_bytes())?;
                        stream.write(&compressed)?;
                    }else {
                        stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
                    }
                }else {
                    println!("P1");
                    stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
                }
                
            }else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        },
        "POST" => {
            let length_finder : Vec<&str> = lines[2].split(" ").collect();
            let my_int = length_finder[1].parse::<i32>().unwrap();
            if tokens[1].starts_with("/files/") {
                let filename = tokens[1].replace("/files/", "");
                let env_args: Vec<_> = env::args().collect();
                println!("Arguments: {:?}", env_args);
                let mut filepath=env_args[2].clone();
                filepath.push('/');
                filepath.push_str(&filename);
                let file = File::create_new(&filepath);
                let response;
                match file {
                    Ok(mut f) => {
                        f.write_all(lines[5][0..my_int as usize].as_bytes())?;
                        response=format!("HTTP/1.1 201 Created\r\n\r\n")
                    }
                    Err(_) => response=format!("HTTP/1.1 404 Not Found\r\n\r\n"),
                }
                stream.write(response.as_bytes())?;
            }
        },
        _ => println!("Method not Found: {:?}", tokens[0])

    }
    Ok(())
    
}
fn get_gzip(data: String) -> (Vec<u8>, usize) {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    let len = compressed.len();
    (compressed, len)
}