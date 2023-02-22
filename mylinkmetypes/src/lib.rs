use linkme::distributed_slice;
use std::collections::HashMap;

pub use mylinkmemacros::*;

#[distributed_slice]
pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> &dyn HttpResponseTrait] = [..];
// alternative:
// pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> HttpResponse] = [..];

pub trait HttpResponseTrait {
    fn to_response(&self) -> String;
}

impl HttpResponseTrait for &str {
    fn to_response(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub code: u16,
    pub body: String,
}

impl Into<HttpResponse> for &str {
    fn into(self) -> HttpResponse {
        HttpResponse { code: 200, body: self.to_string() }
    }
}

pub fn execute_handlers() {
    for handler in GET_HANDLERS {
        // let boxed_response = ;
        println!("Handler return: {:?}", handler(&"body", HashMap::new()).to_response()); 
    }
}
