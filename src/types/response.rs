use super::Headers;

#[derive(Debug)]
pub struct Response {
    pub body: String,
    pub code: u16,
    pub headers: Headers,
    pub protocol: String,
}

impl Response {
    pub fn new(code: u16, headers: Headers, body: String) -> Self {
        Response {
            body,
            code,
            headers,
            protocol: String::from("HTTP/1.1"),
        }
    }

    pub fn reason(&self) -> String {
        String::from(match self.code {
            200 => "OK",
            404 => "NOT FOUND",
            _ => "INDETERMINATE REASON",
        })
    }
}
