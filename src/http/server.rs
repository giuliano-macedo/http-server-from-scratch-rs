use super::{ReadFrom, Request, Response, WriteTo};
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Instant;

type Callback = fn(&Request) -> Response;

enum Handler {
    Callback(Callback),
    File(String),
}

pub struct Server {
    addr: SocketAddr,
    handlers: HashMap<String, Handler>,
}

enum ServeError {
    StartConnection,
    GetClientIp,
    RequestRead(SocketAddr, &'static str),
    ResponseRead(SocketAddr, &'static str),
}

impl fmt::Display for ServeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServeError::StartConnection => write!(f, "couldn't start client connection"),
            ServeError::GetClientIp => write!(f, "couldn't get client ip address"),
            ServeError::RequestRead(ip, err) => {
                write!(f, "couldn't read request from {} because '{}'", ip, err)
            }
            ServeError::ResponseRead(ip, err) => {
                write!(f, "couldn't write response to {} because '{}'", ip, err)
            }
        }
    }
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            addr: SocketAddr::new(
                IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap()),
                port.parse::<u16>().unwrap(),
            ),
            handlers: HashMap::new(),
        }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();

        println!("Listening on http://{}.", self.addr);

        for stream in listener.incoming() {
            match self.serve(stream) {
                Ok(_) => (),
                Err(e) => {
                    println!("[Error] {}", e)
                }
            }
        }
    }

    fn serve(&self, stream: Result<TcpStream, std::io::Error>) -> Result<(), ServeError> {
        let start = Instant::now();

        let mut client = stream.or(Err(ServeError::StartConnection))?;
        let client_ip = client.local_addr().or(Err(ServeError::GetClientIp))?;
        let mut reader = BufReader::with_capacity(4000, &mut client);
        let req = Request::read_from(&mut reader)
            .or_else(|e| Err(ServeError::RequestRead(client_ip, e)))?;
        let res = self.handle_request(&req);
        res.write_to(&mut client)
            .or_else(|e| Err(ServeError::ResponseRead(client_ip, e)))?;

        let duration = start.elapsed();

        println!(
            "[{}] {{{}}} {:?} '{}' -> {} {:.2}ms",
            client_ip,
            req.headers.user_agent().unwrap_or(&String::from("None")),
            req.method,
            req.path,
            res.status as usize,
            duration.as_nanos() as f64 / 1e+6
        );
        Ok(())
    }

    pub fn on(&mut self, pat: &str, cb: Callback) {
        self.handlers.insert(pat.to_string(), Handler::Callback(cb));
    }
    pub fn on_file(&mut self, pat: &str, fname: &str) {
        self.handlers
            .insert(pat.to_string(), Handler::File(fname.to_string()));
    }

    fn handle_request(&self, req: &Request) -> Response {
        let handler = match self.handlers.iter().find(|(k, _)| req.path == **k) {
            Some((_, v)) => v,
            None => return Response::not_found(),
        };
        match handler {
            Handler::Callback(cb) => cb(req),
            Handler::File(fname) => Response::file(fname),
        }
    }
}
