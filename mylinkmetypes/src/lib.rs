use linkme::distributed_slice;
use std::collections::HashMap;

pub use mylinkmemacros::*;

/// alternative which would work for the user-side, but we can't work with `!Sized` return values.
/// pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> dyn HttpResponseTrait] = [..];
/// 
/// alternative that doesn't work user-side because we then you wouldn't be able to return objects
/// owned by the handler function, like `String`s.
/// pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> &dyn HttpResponseTrait] = [..];
/// 
/// alternative which works, but needs to box everything, and uses `dyn`.
/// This isn't great, because boxing everything slows it down, and the vtable which is needed for
/// `dyn HttpResponseTrait` would also mean taking a hit on performance.
/// pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> Box<dyn HttpResponseTrait>] = [..];
/// 
/// alternative that actually works:
/// This works, but you need to call .into() at the end of the handler.
#[distributed_slice]
pub static GET_HANDLERS: [fn(&dyn ToString, HashMap<String, String>) -> HttpResponse] = [..];

pub trait HttpResponseTrait {
    fn to_response(&self) -> String;
}

impl HttpResponseTrait for &str {
    fn to_response(&self) -> String {
        self.to_string()
    }
}

impl HttpResponseTrait for String {
    fn to_response(&self) -> String {
        self.clone()
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

impl Into<HttpResponse> for String {
    fn into(self) -> HttpResponse {
        HttpResponse { code: 200, body: self }
    }
}

pub fn execute_handlers() {
    for handler in GET_HANDLERS {
        println!("Handler return: {:?}", handler(&"body", HashMap::new()));
    }
}
