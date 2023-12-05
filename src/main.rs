use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use book_web_server::{*, HttpStatus, RequestType};

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
    let response = match get_request_type(&request) {
        RequestType::Root => build_response(HttpStatus::Ok, "pages/hello.html"),
        RequestType::Sleep => {
            thread::sleep(Duration::from_secs(5));
            build_response(HttpStatus::Ok, "pages/hello.html")
        },
        RequestType::Unknown => build_response(HttpStatus::NotFound, "pages/404.html")
    };
    stream.write_all(response.as_bytes()).unwrap();
}



fn build_response(status_code: HttpStatus, page_path: &str) -> String {
    let status_line = format!("HTTP/1.1 {} {}", status_code.as_u16(), status_code.as_str());
    let contents = fs::read_to_string(page_path).unwrap();
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}
