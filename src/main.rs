// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
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

    let mut folder_path = String::new();
    let mut found = false;
    for argument in env::args() {
        if argument == "--directory" {
            found = true
        } else if found {
            folder_path = argument;
        }
    }
    println!("{}", folder_path);
    if path.starts_with("/files/") {
        let filename = path.trim_start_matches("/files/");
        let mut path = PathBuf::new();
        path.push(folder_path);
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
    } else if path == "/user-agent" {
        let headers: HashMap<&str, &str> = string_content
            .lines()
            .skip(1)
            .filter_map(|line| return line.split_once(": "))
            .collect();
        let user_agent = headers.get("User-Agent").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            user_agent.as_bytes().len(),
            user_agent
        );

        println!("response: {}", response);
        stream
            .write(response.as_bytes())
            .expect("Error: writing to the stream");
    } else if path.starts_with("/echo/") {
        let echo_str = path.trim_start_matches("/echo/");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            echo_str.as_bytes().len(),
            echo_str
        );

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
