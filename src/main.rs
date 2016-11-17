use std::io::{self, BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::process;

const CLIPBOARD_MAX_SIZE: usize = 4096 * 10;
const BUFFER_SIZE: usize = 4096;
const CLIPBOARD_DEFAULT_FILE: &'static str = "/tmp/sticky-default";

fn handle_get() -> io::Result<()> {
    println!("Content-type: text/plain");
    println!("");

    let clipboard_file = try!(File::open(CLIPBOARD_DEFAULT_FILE));
    let mut clipboard = BufReader::new(clipboard_file);

    match clipboard.fill_buf() {
        Ok(buffer) => {
            try!(io::stdout().write(buffer));
        },
        Err(_) => ()
    }
    Ok(())
}

fn handle_post() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = [0; BUFFER_SIZE];
    let mut clipboard: Vec<u8> = Vec::new();
    let mut clipboard_file = try!(File::create(CLIPBOARD_DEFAULT_FILE));

    println!("Content-type: text/plain");
    println!("");

    while try!(stdin.read(&mut buffer[..])) > 0 {
        clipboard.extend(buffer.iter());
        if clipboard.len() > CLIPBOARD_MAX_SIZE {
            break;
        }
    }
    try!(clipboard_file.write_all(&clipboard));

    Ok(())
}

fn main() {
    let result = match env::var("REQUEST_METHOD") {
        Ok(val) => match val.as_str() {
            "GET" => handle_get(),
            "POST" => handle_post(),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Unknown/Unsupported request method")),
        },
        Err(_) => Err(Error::new(ErrorKind::InvalidInput, "REQUEST_METHOD not set")),
    };

    process::exit(if result.is_ok() {0} else {1});
}
