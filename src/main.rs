use std::io::{self, Error, ErrorKind};
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::process;

const CLIPBOARD_MAX_SIZE: usize = 40 * 1024 * 1024;
const BUFFER_SIZE: usize = 4096;
const CLIPBOARD_DEFAULT_FILE: &'static str = "/tmp/sticky-default";
const REQUEST_METHOD: &'static str = "REQUEST_METHOD";
const RM_POST: &'static str = "POST";
const RM_GET: &'static str = "GET";

struct StickySettings {
    cgi_mode: bool,
}

impl StickySettings {
    fn new() -> StickySettings {
        StickySettings {
            cgi_mode: true,
        }
    }
}

fn handle_get(settings: &StickySettings) -> io::Result<()> {
    if settings.cgi_mode {
        println!("Content-type: application/octet-stream");
        println!("");
    }

    let handle = io::stdout();
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut clipboard_file = try!(File::open(CLIPBOARD_DEFAULT_FILE));
    let mut stdout = handle.lock();

    while let Ok(nbytes) = clipboard_file.read(&mut buffer[..]) {
        if nbytes > 0 {
            try!(stdout.write(&buffer[..nbytes]));
        } else {
            break;
        }
    }

    Ok(())
}

fn handle_post(settings: &StickySettings) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = [0; BUFFER_SIZE];
    let mut clipboard: Vec<u8> = Vec::new();
    let mut clipboard_file = try!(File::create(CLIPBOARD_DEFAULT_FILE));

    if settings.cgi_mode {
        println!("Content-type: text/plain");
        println!("");
    }

    while let Ok(nbytes) = stdin.read(&mut buffer[..]) {
        if nbytes > 0 {
            clipboard.extend_from_slice(&buffer[..nbytes]);
            if clipboard.len() > CLIPBOARD_MAX_SIZE {
                println!("too much data, cropping");
                break;
            }
        } else {
            break;
        }
    }
    try!(clipboard_file.write_all(&clipboard));

    Ok(())
}

fn main() {
    let mut settings = StickySettings::new();

    for argument in env::args() {
        match argument.as_str() {
            "--cli" | "-c" => settings.cgi_mode = false,
            "--post" | "-p" => env::set_var(REQUEST_METHOD, RM_POST),
            "--get" | "-g" => env::set_var(REQUEST_METHOD, RM_GET),
            _ => (),
        }
    }

    let result = match env::var(REQUEST_METHOD) {
        Ok(val) => match val.as_str() {
            RM_GET => handle_get(&settings),
            RM_POST => handle_post(&settings),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Unknown/Unsupported request method")),
        },
        Err(_) => Err(Error::new(ErrorKind::InvalidInput, "REQUEST_METHOD not set")),
    };

    process::exit(if result.is_ok() {0} else {1});
}
