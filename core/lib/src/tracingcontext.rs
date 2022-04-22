use std::fmt;

use crate::Request;

pub struct TracingContext(String);

impl TracingContext {
    pub fn new(req: &Request<'_>) -> Self {
        if let Some(header) = req.headers().get_one("tracing") {
            return Self(header.to_string());
        } else if let Some(header) = req.headers().get_one("x-request-id") {
            return Self(header.to_string());
        }
        Self("none".to_string())
    }    
}

impl fmt::Display for TracingContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
