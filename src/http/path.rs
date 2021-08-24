use super::utils::parse_url_param;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
pub struct Path {
    data: String,
    params: Option<String>,
}

impl Path {
    pub fn parse_params(&self) -> Result<HashMap<&str, &str>, &'static str> {
        match &self.params {
            Some(p) => parse_url_param(&p[..]),
            None => Ok(HashMap::new()),
        }
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let (data, params) = match s.split_once('?') {
            Some((d, p)) => (d.to_string(), Some(p.to_string())),
            None => (s.to_string(), None),
        };
        Path { data, params }
    }
}

impl std::cmp::PartialEq<String> for Path {
    fn eq(&self, other: &String) -> bool {
        self.data == *other
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.params {
            Some(p) => write!(f, "{}?{}", &self.data, p),
            None => write!(f, "{}", &self.data),
        }
    }
}
