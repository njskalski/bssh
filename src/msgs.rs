use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

use regex::bytes::Regex;
use bssh_err;

const buffer_length : usize = 255;

pub fn read_welcome_string(mut stream : TcpStream, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : [u8; buffer_length];
    let mut res : Vec<String> = Vec::new();

    let eol = Regex::new(r"\r\n").unwrap();

    loop {
        buf = [0 as u8; buffer_length];
        stream.read_exact(&mut buf);
        let line = match eol.find(&buf) {
            None => panic!(bssh_err::BSSH_ERR_NO_DELIMITER_FOUND),
            Some(idx) => buf[..idx.start()].to_vec()
        };



        break
    }

    Ok(res)
}
