use std::collections::HashMap;

pub mod request;
pub mod response;

pub use self::{
    request::Request,
    response::Response,
};

type Headers = HashMap<String, String>;
