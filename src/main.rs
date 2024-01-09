use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use http::{methods::Method, request::Request};
use nom::Slice;

mod http;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client with address: {addr} connected");
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("Couldn't get client: {:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";
    let ok_response = "HTTP/1.1 200 OK\r\n\r\n";
    let created_response = "HTTP/1.1 201 Created\r\n\r\n";

    let mut buffer = [0u8; 1024];
    // reading from stream to the buffer
    stream.read(&mut buffer).expect("ERROR: reading stream");
    let string_content = String::from_utf8(buffer.to_vec()).unwrap();

    let mut directory_path = String::from("assets");
    for (i, argument) in env::args().enumerate() {
        if argument == "--directory" && i + 1 < env::args().len() {
            directory_path = env::args().nth(i + 1).unwrap();
        }
    }

    let request = Request::from(string_content);
    if request.path.starts_with("/files/") {
        match request.method {
            // for posting file
            Method::Post => {
                let request_body = request.body.unwrap();
                let filename = request.path.trim_start_matches("/files/");
                let mut path = PathBuf::new();
                path.push(directory_path);
                path.push(filename);
                let content_length: usize = request
                    .headers
                    .get("Content-Length")
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut file = File::create(path).expect("error creating file");
                let file_bytes = request_body.as_bytes().slice(0..content_length);

                file.write(file_bytes).unwrap();
                stream.write(created_response.as_bytes()).unwrap();
            }
            // for getting file
            Method::Get => {
                let filename = request.path.trim_start_matches("/files/");
                let mut path = PathBuf::new();
                path.push(directory_path);
                path.push(filename);
                match fs::read_to_string(path) {
                    Ok(file_str) => {
                        let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n",
                    file_str.as_bytes().len(),
                    file_str
                );
                        stream
                            .write(response.as_bytes())
                            .expect("Error: writing to the stream");
                    }
                    Err(_) => {
                        stream.write(not_found_response.as_bytes()).unwrap();
                        println!("accepted new connection");
                    }
                }
            }
        }
    } else if request.path == "/user-agent" {
        let user_agent = request.headers.get("User-Agent").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            user_agent.as_bytes().len(),
            user_agent
        );

        println!("response: {}", response);
        stream
            .write(response.as_bytes())
            .expect("Error: writing to the stream");
    } else if request.path.starts_with("/echo/") {
        let echo_str = request.path.trim_start_matches("/echo/");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            echo_str.as_bytes().len(),
            echo_str
        );

        println!("response: {}", response);
        stream
            .write(response.as_bytes())
            .expect("Error: writing to the stream");
    } else if request.path != "/" {
        stream.write(not_found_response.as_bytes()).unwrap();
    } else {
        stream.write(ok_response.as_bytes()).unwrap();
    }
}
