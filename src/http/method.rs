use std::convert::From;
#[derive(Debug, PartialEq)]
pub enum Method {
    // The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    GET,
    // The HEAD method asks for a response identical to that of a GET request, but without the response body.
    HEAD,
    // The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
    POST,
    // The PUT method replaces all current representations of the target resource with the request payload.
    PUT,
    // The DELETE method deletes the specified resource.
    DELETE,
    // The CONNECT method establishes a tunnel to the server identified by the target resource.
    CONNECT,
    // The OPTIONS method is used to describe the communication options for the target resource.
    OPTIONS,
    // The TRACE method performs a message loop-back test along the path to the target resource.
    TRACE,
    // The PATCH method is used to apply partial modifications to a resource.
    PATCH,
}

impl From<&str> for Method {
    fn from(input: &str) -> Self {
        match input {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            unrecognized => {
                println!(
                    "Warning: unrecognized method in request '{}', using GET",
                    unrecognized
                );
                Method::GET
            }
        }
    }
}
