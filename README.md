# HTTP Server from scratch in Rust
This project is a zero dependency multi-threaded HTTP server made from scratch with headers and some file extensions support in Rust.
I wrote this code to assit my understanding in Rust and the HTTP protocol, it was inspired by the lessons in the
[udemy course](https://www.udemy.com/course/rust-fundamentals/) made by Lyubomir Gavadinov.

**Note**: This implementation is not meant to be deployed in a production environment.

### Pre-requirements
* Cargo

### Running

Just use the following command in the root of the repo to run in `http://0.0.0.0:1234` with 32 threads:
```bash
cargo run 1234 32
```

and then access `http://0.0.0.0:1234` to be redirected to index.html.