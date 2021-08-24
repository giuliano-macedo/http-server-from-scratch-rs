// use std::fmt;
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    OK = 200,
    // Permanent Redirect
    REDIRECT = 301,
    // Service Unavailable
    UNAVAILABLE = 503,
    // Internal Server Error
    INTERNALERR = 500,
    // Not Found
    NOTFOUND = 404,
}
