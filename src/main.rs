use mylinkmetypes::{execute_handlers, get, HttpResponse};
use std::collections::HashMap;

#[get]
fn handler_1(input: &dyn ToString, headers: HashMap<String, String>) -> HttpResponse {
    println!(
        "From inside handler_1 again, {:?}, {:?}",
        input.to_string(),
        headers
    );
    let body = input.to_string();
    body.into()
    // alternative:
    // &""
    // alternative:
    // HttpResponse { code: 400, body: "testbody".into() }
}

fn main() {
    execute_handlers();
}
