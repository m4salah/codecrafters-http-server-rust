use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use http::{
    methods::Method,
    request::Request,
    response::{HttpStatus, Response},
};
use nom::{AsBytes, Slice};

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

                let response = Response::new(HttpStatus::Created);
                stream.write(response.into_response().as_bytes()).unwrap();
            }
            // for getting file
            Method::Get => {
                let filename = request.path.trim_start_matches("/files/");
                let mut path = PathBuf::new();
                path.push(directory_path);
                path.push(filename);
                match fs::read_to_string(path) {
                    Ok(file_str) => {
                        let response = Response::new(HttpStatus::Ok)
                            .add_header(
                                "Content-Type".to_string(),
                                "application/octet-stream".to_string(),
                            )
                            .add_header(
                                "Content-Length".to_string(),
                                file_str.as_bytes().len().to_string(),
                            )
                            .set_body(file_str);
                        stream.write(response.into_response().as_bytes()).unwrap();
                    }
                    Err(_) => {
                        let response = Response::new(HttpStatus::NotFound);
                        stream.write(response.into_response().as_bytes()).unwrap();
                        println!("accepted new connection");
                    }
                }
            }
        }
    } else if request.path == "/user-agent" {
        let user_agent = request.headers.get("User-Agent").unwrap();

        let response = Response::new(HttpStatus::Ok)
            .add_header("Content-Type".to_string(), "text/plain".to_string())
            .add_header(
                "Content-Length".to_string(),
                user_agent.as_bytes().len().to_string(),
            )
            .set_body(user_agent.to_string());
        stream.write(response.into_response().as_bytes()).unwrap();
    } else if request.path.starts_with("/echo/") {
        let echo_str = request.path.trim_start_matches("/echo/");

        let mut response = Response::new(HttpStatus::Ok)
            .add_header("Content-Type".to_string(), "text/plain".to_string())
            .add_header(
                "Content-Length".to_string(),
                echo_str.as_bytes().len().to_string(),
            )
            .set_body(echo_str.to_string());
        if let Some(gzip) = request.headers.get("Accept-Encoding")
            && gzip == &"gzip"
        {
            response = response.add_header("Content-Encoding".to_string(), "gzip".to_string());
        }
        stream.write(response.into_response().as_bytes()).unwrap();
    } else if request.path != "/" {
        let response = Response::new(HttpStatus::NotFound);
        stream.write(response.into_response().as_bytes()).unwrap();
    } else {
        let response = Response::new(HttpStatus::Ok);
        stream.write(response.into_response().as_bytes()).unwrap();
    }
}
