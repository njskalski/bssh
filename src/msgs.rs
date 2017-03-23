use std::{env, error};
use std::net::TcpStream;
use std::io::Read;

use regex::bytes::Regex;
use bssh_err;

const max_buffer_length : usize = 255; //RFC 4253 page 5
const max_comments_lines : usize = 10; //TODO arbitrary value

pub fn read_welcome_string(stream : &mut Read, allow_comments : bool) -> Result<Vec<String>, Box<error::Error + Send + Sync>> {
    let mut buf : Vec<u8> = Vec::new();
    let mut res : Vec<String> = Vec::new();
    let eol = Regex::new(r"\r\n").unwrap();

    buf.reserve(max_buffer_length + 1); // +1 so I can add first, and then check if len() > max_buffer_length

    loop {
        let mut byte_buf = [0 as u8; 1];
        stream.read_exact(&mut byte_buf);
        buf.push(byte_buf[0]);

        //protocol specifies '\r\n' as end of line
        if buf.len() >= 2 && buf[buf.len()-2] == '\r' as u8 && buf[buf.len()-1] == '\n' as u8 {

            let line_str : String = match String::from_utf8(buf) {
                Err(_) => panic!(bssh_err::BSSH_ERR_NOT_UTF8_STRING),
                Ok(s) => s
            };
            buf = Vec::new();

            res.push(line_str);

            if res.last().unwrap().starts_with("SSH-2.0-") {
                break;
            } else {
                if !allow_comments {
                    panic!(bssh_err::BSSH_ERR_EXPECTED_HEADER_STRING);
                };

                if res.len() > max_comments_lines {
                    panic!(bssh_err::BSSH_ERR_TOO_MANY_COMMENT_LINES);
                };
            };

        } else {
            if buf.len() > max_buffer_length {
                panic!(bssh_err::BSSH_ERR_NO_LINE_TERMINATION_FOUND);
            }
        };
    };

    Ok(res)
}

#[cfg(test)]
mod tests {

    struct MockStream {
        input : Vec<u8>
    }

    impl io::Read for MocStream {

    }

    fn test_read_welcome_string_accepts_simple_string() {


    }
}
