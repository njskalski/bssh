use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

use bssh_err;

const buffer_length : u32 = 255;

pub fn read_welcome_string(stream : TcpStream, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : [u8; buffer_length];
    let mut res : Vec<String>;

    loop {
        buf = [0 as u8; buffer_length];
        stream.read_exact(&mut buf);
        let line = match buf.find("\r\n") {
            None => panic!(bssh_err::BSSH_ERR_NO_DELIMITER_FOUND),
            Some(idx) => buf[..idx].to_vec()
        };
        break
    }

    Ok(res)
}
