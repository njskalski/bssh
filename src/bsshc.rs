use std::{env, error};
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;

extern crate regex;

mod version;
mod msgs;
mod bssh_err;

const HOST: &'static str = "127.0.0.1:5555";

fn send_one_command(command: &Vec<u8>) -> Result<(), Box<error::Error + Send + Sync>> {

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&command));

    let mut response = String::new();
    stream.read_to_string(&mut response);
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    println!("{:?}", response);

    Ok(())
}

fn main() {
    let hello : Vec<u8> = [version::get_version_byte_string(),  b"\r\n".to_vec()].concat();

    match send_one_command(&hello) {
        Ok(_) => println!("ok"),
        Err(err) => println!("An error occurred: {}", err),
    }
}
