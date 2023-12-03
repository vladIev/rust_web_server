use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

type Request = Vec<String>;

enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    fn as_u16(&self) -> u16 {
        match *self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
        }
    }

    fn as_str(&self) -> &'static str {
        match *self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "NOT FOUND",
        }
    }
}

fn main() {
    let default_address = "127.0.0.1";
    let default_port = 7878;
    let listener = TcpListener::bind(format!("{default_address}:{default_port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = get_request(buf_reader);
    let response = if is_root_request(&request) {
        build_response(HttpStatus::Ok, "pages/hello.html")
    } else {
        build_response(HttpStatus::NotFound, "pages/404.html")
    };
    stream.write_all(response.as_bytes()).unwrap();
}

fn get_request<R: Read>(buf_reader: BufReader<R>) -> Request {
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn is_root_request(request: &Request) -> bool {
    let request_line = get_request_line(request);
    request_line == "GET / HTTP/1.1"
}

fn get_request_line(request: &Request) -> &str {
    &request[0]
}

fn build_response(status_code: HttpStatus, page_path: &str) -> String {
    let status_line = format!("HTTP/1.1 {} {}", status_code.as_u16(), status_code.as_str());
    let contents = fs::read_to_string(page_path).unwrap();
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}
