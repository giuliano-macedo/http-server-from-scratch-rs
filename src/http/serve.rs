use super::{ReadFrom, Request, Router, WriteTo};
use std::convert::From;
use std::fmt;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream};
use std::time::Instant;

pub type StreamType = Result<(TcpStream, SocketAddr), std::io::Error>;

pub enum ServeError {
    StartConnection,
    RequestRead(SocketAddr, &'static str),
    ResponseRead(SocketAddr, &'static str),
}

impl fmt::Display for ServeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServeError::StartConnection => write!(f, "couldn't start client connection"),
            ServeError::RequestRead(ip, err) => {
                write!(f, "couldn't read request from {} because '{}'", ip, err)
            }
            ServeError::ResponseRead(ip, err) => {
                write!(f, "couldn't write response to {} because '{}'", ip, err)
            }
        }
    }
}

pub fn serve(thread_name: &str, router: &Router, stream: StreamType) -> Result<(), ServeError> {
    let start = Instant::now();

    let (mut client, client_ip) = stream.or(Err(ServeError::StartConnection))?;

    let mut reader = BufReader::with_capacity(4000, &mut client);
    let req =
        Request::read_from(&mut reader).or_else(|e| Err(ServeError::RequestRead(client_ip, e)))?;
    let res = router.handle_request(&req);
    res.write_to(&mut client)
        .or_else(|e| Err(ServeError::ResponseRead(client_ip, e)))?;

    let duration = start.elapsed();

    println!(
        "#{} [{}] {{{}}} {:?} '{}' -> {} {:.2}ms",
        thread_name,
        client_ip,
        req.headers.user_agent().unwrap_or(&String::from("None")),
        req.method,
        req.path,
        res.status as usize,
        duration.as_nanos() as f64 / 1e+6
    );
    Ok(())
}
