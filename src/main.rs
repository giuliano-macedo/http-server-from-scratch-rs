mod http;
use http::{parse_url_param, Method, Response, Server};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("[USAGE] {} <PORT> <NO_THREADS>", args[0]);
    }
    let no_threads:usize = args[2].parse().unwrap();
    
    let mut server = Server::new("0.0.0.0", &args[1]);
    server.on_file(r"/index.html", "./static/index.html");
    server.on_file(r"/favicon.png", "./static/favicon.png");
    server.on_file(r"/js/main.js", "./static/js/main.js");
    server.on_file(r"/img/rust-logo-blk.svg", "./static/img/rust-logo-blk.svg");
    server.on_file(r"/css/main.css", "./static/css/main.css");
    server.on(r"/", |_| Response::redirect("/index.html"));
    server.on(r"/🦀", |_| Response::redirect("/index.html"));
    server.on(r"/%F0%9F%A6%80", |_| Response::redirect("/index.html"));

    server.on(r"/api/sum", |req| {
        if req.method != Method::GET && req.method != Method::POST {
            return Response::internal_err("Only get or post baby");
        }
        let mut query = match req.path.parse_params() {
            Ok(q) => q,
            Err(_) => return Response::internal_err("Couldn't parse query"),
        };
        let body = match parse_url_param(&req.body) {
            Ok(b) => b,
            Err(_) => return Response::internal_err("Couldn't parse body"),
        };

        query.extend(body);

        let x = match query.get("x").unwrap_or(&"0").parse::<f64>() {
            Ok(v) => v,
            Err(_) => return Response::internal_err("Couldn't parse x"),
        };
        let y = match query.get("y").unwrap_or(&"0").parse::<f64>() {
            Ok(v) => v,
            Err(_) => return Response::internal_err("Couldn't parse y"),
        };

        let ans = (x + y).to_string() + "\n";
        Response::ok(&ans)
    });
    server.run(no_threads)
}
