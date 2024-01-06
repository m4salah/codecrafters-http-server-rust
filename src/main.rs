// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 1024];
                stream.read(&mut buffer).unwrap();
                let string_content = String::from_utf8_lossy(&buffer);

                let mut spli = string_content.split(' ');
                spli.next().unwrap();
                if spli.next().unwrap() != "/" {
                    let ok_response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write(ok_response.as_bytes()).unwrap();
                    println!("accepted new connection");
                } else {
                    let ok_response = "HTTP/1.1 200 OK\r\n\r\n";
                    stream.write(ok_response.as_bytes()).unwrap();
                    println!("accepted new connection");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
