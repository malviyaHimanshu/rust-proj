use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // update buffer by incoming request data
    stream.read(&mut buffer).unwrap();

    // write back the response
    let response_content = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_content.len(),
        response_content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    // creating a local tcp listener which listens to any incoming tcp connections
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("we here, listening at 127.0.0.1:3000");

    // connecting to any incoming connections
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection established!");
        // logic to handle incoming connections
        handle_connection(_stream);
    }
}