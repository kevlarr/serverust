use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use super::Headers;

#[derive(Debug)]
pub struct Request {
    pub headers: Headers,
    pub method: String,
    pub path: String,
    protocol: String,
}

impl Request {
    pub fn from_stream(stream: &TcpStream) -> Self {
        let mut buf = BufReader::new(stream);

        let (method, path, protocol) = Self::parse_start_line(&mut buf);
        let headers = Self::parse_headers(&mut buf);

        Request {
            path,
            protocol,
            method,
            headers,
        }
    }

    fn parse_start_line<T: Read>(buf: &mut BufReader<T>) -> (String, String, String) {
        let mut start_line = String::new();
        buf.read_line(&mut start_line)
            .expect("Reading start line shouldn't fail");

        let parts = start_line.trim().split(' ').collect::<Vec<_>>();
        (parts[0].to_string(), parts[1].to_string(), parts[2].to_string())
    }

    fn parse_headers<T: Read>(buf: &mut BufReader<T>) -> HashMap<String, String> {
        // Read lines to extract headers, until there's a blank
        let mut headers = HashMap::new();

        for line in buf.lines() {
            match line {
                Ok(l) => match l.trim() {
                    blank if blank == "" => break,
                    header => {
                        let header_split = header.split(':').collect::<Vec<_>>();
                        let key = header_split[0].trim().to_string();
                        let value = header_split[1].trim().to_string();

                        headers.insert(key, value);
                    },

                },
                Err(e) => panic!(e),
            }
        }

        headers
    }
}
