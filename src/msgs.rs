use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

let buffer_length = 255;

pub fn read_welcome_string(stream : TcpStream, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : [u8; buffer_length];
    let mut res : Vec<String>;

    loop {
        buf = [0 as u8; buffer_length];
        stream.read_exact(&mut buf);
        let pos = buf.find("\r\n");
        break;
    }

    Ok(res)
}
