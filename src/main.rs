use std::io;
use std::io::prelude::*;
use std::env;

fn handle_get() {
    println!("Content-type: text/plain");
    println!("");
    println!("This is my boomstick!");
}

fn main() {
    match env::var("REQUEST_METHOD") {
        Ok(val) => match val.as_str() {
            "GET" => handle_get(),
            _ => (),
        },
        Err(e) => println!("REQUEST_METHOD not set: {}", e),
    }
    /*
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        println!("line - {}", line.unwrap());
    }
    */
}
