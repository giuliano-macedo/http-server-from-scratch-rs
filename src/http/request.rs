use super::{Headers, Method, Path, ReadFrom};
use std::convert::TryFrom;
use std::io::BufRead;

pub struct Request {
    pub method: Method,
    pub path: Path,
    pub headers: Headers,
    pub body: String,
}

impl ReadFrom for Request {
    type Error = &'static str;
    fn read_from<R: BufRead>(stream: &mut R) -> Result<Self, Self::Error> {
        let mut buff: [u8; 4000] = [0; 4000];
        let l = stream.read(&mut buff).or(Err("failed receiving request"))?;
        let buff_str = String::from_utf8_lossy(&buff[0..l]);
        let mut buff_str_splitted = buff_str.split('\n');

        let first_line = match buff_str_splitted.next() {
            Some(v) => v,
            None => return Err("Request have only one line!"),
        };

        let mut first_line_splitted = first_line.split(' ');

        let method = Method::from(match first_line_splitted.next() {
            Some(v) => v,
            None => return Err("First line has no spaces!"),
        });
        let path = match first_line_splitted.next() {
            Some(v) => Path::from(v),
            None => return Err("First line doesn't have a path!"),
        };
        let headers =
            Headers::try_from(&mut buff_str_splitted).or(Err("Failed parsing headers"))?;

        // WARNING: Eventually rust will have intersperse in iterator that adds delimiters to iterators so that it can be collected into a string
        // https://doc.rust-lang.org/1.54.0/std/iter/trait.Iterator.html#method.intersperse
        // let body = buff_str_splitted.intersperse("\n").collect();
        let body = buff_str_splitted.collect::<Vec<&str>>().join("\n");

        return Ok(Self {
            method,
            path,
            headers,
            body,
        });
    }
}
