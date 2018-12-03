extern crate serverust;

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use serverust::types::{Request, Response};

fn main() {
    let addr = "127.0.0.1:1337";
    let listener = TcpListener::bind(addr).unwrap();
    println!("serverust now listening on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    println!("Received stream: {:?}", stream);

    let req = Request::from_stream(&stream);
    println!("{:?}", req);

    let hello = fs::read_to_string("hello.html").unwrap();
    let resp = Response::new(200, HashMap::new(), hello);
    println!("{:?}", resp);

    let text = format!("{} {} {}\r\n\r\n{}",
        resp.protocol,
        resp.code,
        resp.reason(),
        //resp.headers,
        resp.body,
    );

    stream.write(text.as_bytes()).unwrap();
    stream.flush().unwrap();
}
