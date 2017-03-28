use std::error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;

extern crate bsshlib;

use bsshlib::version;
use bsshlib::msgs;

const HOST: &'static str = "127.0.0.1:5555";

fn connect() -> Result<(), Box<error::Error + Send + Sync>> {

    let hello: Vec<u8> = [version::get_version_byte_string(), b"\r\n".to_vec()].concat();

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&hello));

    msgs::read_welcome_string(&mut stream, true);

    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");

    Ok(())
}

fn main() {

    match connect() {
        Ok(_) => println!("ok"),
        Err(err) => println!("An error occurred: {}", err),
    }
}
