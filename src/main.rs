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

    let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";
    let ok_response = "HTTP/1.1 200 OK\r\n\r\n";

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 1024];
                // reading from stream to the buffer
                stream.read(&mut buffer).unwrap();
                let string_content = String::from_utf8_lossy(&buffer);

                println!("{}", string_content);
                let mut spli = string_content.split(' ').skip(1).take(1);

                // &str vs String
                // &str is static size string in stack
                // String dynamic string in the heap
                let path = spli.next().unwrap();

                if path.starts_with("/echo/") {
                    let echo_str = path.split("/echo/").skip(1).next().unwrap();
                    let mut response = String::from("HTTP/1.1 200 OK\r\n");
                    response.push_str("Content-Type: text/plain\r\n");
                    response.push_str(&format!("Content-Length: {}\r\n", echo_str.len()));
                    response.push_str(&echo_str);
                    response.push_str("\r\n\r\n");
                    stream.write(response.as_bytes()).unwrap();
                } else if path != "/" {
                    stream.write(not_found_response.as_bytes()).unwrap();
                    println!("accepted new connection");
                } else {
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
