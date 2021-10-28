use super::StatusCode;
use std::{fmt::{Debug, Display, Formatter, Result as FmtResult}, net::TcpStream};
use std::io::{Write, Result as IoResult};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response { status_code, body }
    }
    //dyn will check the impl in runtime using the v table to match each impl to use
    //impl will create multiple functions but just change the type for example instead of write ill be TcpStream or File
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{

        let body = match &self.body{
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phares(),
            body
            
        )
    }
}


