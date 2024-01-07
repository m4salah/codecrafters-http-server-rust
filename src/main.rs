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
                stream.read(&mut buffer).expect("ERROR: reading stream");
                let string_content = String::from_utf8_lossy(&buffer);

                println!("{}", string_content);
                let mut spli = string_content.split(' ').skip(1);

                // &str vs String
                // &str is static size string in stack
                // String dynamic string in the heap
                let path = spli.next().expect("Error: getting next");

                if path.starts_with("/echo/") {
                    let echo_str = path
                        .split("/echo/")
                        .skip(1)
                        .next()
                        .expect("ERROR: echo_str");

                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n", echo_str.as_bytes().len(), echo_str);

                    println!("response: {}", response);
                    stream
                        .write(response.as_bytes())
                        .expect("Error: writing to the stream");
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
