use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

use regex::bytes::Regex;
use bssh_err;

const buffer_length : usize = 255; //RFC 4253 page 5
const max_comments_lines : usize = 10; //TODO arbitrary value

//TODO error is here... we need not to remove shit after \r\n from buffer, but to move it to next iteration.
pub fn read_welcome_string(mut stream : TcpStream, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : [u8; buffer_length];
    let mut res : Vec<String> = Vec::new();
    let eol = Regex::new(r"\r\n").unwrap();

    loop {
        buf = [0 as u8; buffer_length];
        stream.read_exact(&mut buf);
        let line_bytes : Vec<u8> = match eol.find(&buf) {
            None => panic!(bssh_err::BSSH_ERR_NO_LINE_TERMINATION_FOUND),
            Some(idx) => buf[..idx.start()].to_vec()
        };

        let line_str : String = match String::from_utf8(line_bytes) {
            Err(_) => panic!(bssh_err::BSSH_ERR_NOT_UTF8_STRING),
            Ok(s) => s
        };

        res.push(line_str);

        if res.last().unwrap().starts_with("SSH-2.0-") {
            break;
        };

        if !(allow_comments && res.len() < max_comments_lines) {
            return Error("xxx"); //TODO!
        };
    };

    Ok(res)
}
