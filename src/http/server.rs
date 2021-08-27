use super::{Callback, Router, serve};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

pub struct Server {
    addr: SocketAddr,
    router: Router,
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            addr: SocketAddr::new(
                IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap()),
                port.parse::<u16>().unwrap(),
            ),
            router: Router::new(),
        }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();

        println!("Listening on http://{}.", self.addr);

        for stream in listener.incoming() {
            match serve(&self.router, stream) {
                Ok(_) => (),
                Err(e) => {
                    println!("[Error] {}", e)
                }
            }
        }
    }

    pub fn on(&mut self, pat: &str, cb: Callback) {
        self.router.insert_callback(pat, cb);
    }
    pub fn on_file(&mut self, pat: &str, fname: &str) {
        self.router.insert_file(pat, fname);
    }
}


