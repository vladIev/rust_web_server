use std::io::{prelude::*, BufReader};

type Request = Vec<String>;

pub enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    pub fn as_u16(&self) -> u16 {
        match *self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "NOT FOUND",
        }
    }
}

pub enum RequestType {
    Root,
    Sleep,
    Unknown,
}

pub fn get_request<R: Read>(buf_reader: BufReader<R>) -> Request {
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

pub fn get_request_type(request: &Request) -> RequestType {
    let request_line = get_request_line(request);
    match request_line {
        "GET / HTTP/1.1" => RequestType::Root,
        "GET /sleep HTTP/1.1" => RequestType::Sleep,
        _ => RequestType::Unknown,
    }
}

fn get_request_line(request: &Request) -> &str {
    &request[0]
}