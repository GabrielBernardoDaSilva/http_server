use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode{
    pub fn reason_phares(&self) -> &str{
        match self{
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", *self as u16)
    }
}