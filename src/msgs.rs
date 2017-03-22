use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

use bssh_err;

use regex::Regex;

const buffer_length : usize = 255;

pub fn read_welcome_string(mut stream : TcpStream, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : [u8; buffer_length];
    let mut res : Vec<String> = Vec::new();

    let eol = Regex::new(r"\r\n");

    loop {
        buf = [0 as u8; buffer_length];
        stream.read_exact(&mut buf);
        // let line = match buf.windows(2).position(|&x| x == ['\r' as u8, '\n' as u8]) {
        //     None => panic!(bssh_err::BSSH_ERR_NO_DELIMITER_FOUND),
        //     Some(idx) => buf[..idx].to_vec()
        // };



        break
    }

    Ok(res)
}
