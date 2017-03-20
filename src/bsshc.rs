use std::{env, error};
use std::io::prelude::*;
use std::net::TcpStream;

use version;

const HOST: &'static str = "127.0.0.1:5555";

fn send_one_command(command: &Vec<u8>) -> Result<String, Box<error::Error + Send + Sync>> {

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&command));

    let mut response = String::new();
    let mut limited = stream.take(1024);
    try!(limited.read_to_string(&mut response));

    Ok(response)
}

fn main() {
    let hello : Vec<u8> = version::get_version_byte_string() + b"\r\n".to_vec();

    match send_one_command(&hello) {
        Ok(response) => println!("{}", response),
        Err(err) => println!("An error occurred: {}", err),
    }
}
