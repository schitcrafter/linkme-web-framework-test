use std::collections::HashMap;
use mylinkmetypes::{execute_handlers, get, HttpResponse, HttpResponseTrait};

#[get]
fn handler_1(input: &dyn ToString, headers: HashMap<String, String>) -> &dyn HttpResponseTrait {
    println!("From inside handler_1, {:?}, {:?}", input.to_string(), headers);
    &""
    // alternative:
    // HttpResponse { code: 400, body: "testbody".into() }
}

fn main() {
    execute_handlers();
}
