use super::{mime_type, Headers, StatusCode, WriteTo};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

// use lazy_static::lazy_static;
// TODO
// lazy_static!{
//     static ref PLAIN_TEXT_HEADER: Vec<(&'static str, &'static str)> = vec![("Content-Type", "text/plain")];
// }

impl Response {
    pub fn redirect(path: &str) -> Self {
        Self {
            status: StatusCode::REDIRECT,
            headers: Headers::from(&vec![("Content-Type", "text/plain"), ("Location", path)]),
            body: vec![],
        }
    }
    pub fn ok(body: &str) -> Self {
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: body.bytes().collect(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOTFOUND,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: "404\n".bytes().collect(),
        }
    }

    pub fn internal_err(body: &str) -> Self {
        Response {
            status: StatusCode::INTERNALERR,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: body.bytes().collect(),
        }
    }

    pub fn file(path: &str) -> Self {
        let path_buf = PathBuf::from(path);
        let mut f = match File::open(path) {
            Ok(s) => s,
            Err(_) => return Self::not_found(),
        };
        let mut body = Vec::<u8>::new();
        if f.read_to_end(&mut body).is_err() {
            return Self::internal_err("Couldn't read file");
        }
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Type", mime_type(&path_buf))]),
            body,
        }
    }
}

impl WriteTo for Response {
    type Error = &'static str;
    fn write_to<W: Write>(&self, stream: &mut W) -> Result<(), Self::Error> {
        stream
            .write_fmt(format_args!("HTTP/1.1 {}\n", self.status as u32))
            .or(Err("Failed sending status code"))?;
        for (key, value) in self.headers.iter() {
            stream
                .write_fmt(format_args!("{}: {}\n", key, value))
                .or(Err("Failed sending headers data"))?;
        }
        stream
            .write_fmt(format_args!("\n"))
            .or(Err("Failed sending body separator"))?;
        stream
            .write_all(&self.body)
            .or(Err("Failed sending payload"))?;
        Ok(())
    }
}
