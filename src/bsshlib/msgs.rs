use std::io::{Error, ErrorKind, Read, Write};
use rand;
use config;
use errors;
use numbers;
use io_helpers;

const MAX_BUFFER_LENGTH: usize = 255; //RFC 4253 page 5
const MAX_COMMENT_LINES: usize = 10; //TODO arbitrary value

pub fn read_welcome_string(stream: &mut Read, allow_comments: bool) -> Result<Vec<String>, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let mut res: Vec<String> = Vec::new();

    buf.reserve(MAX_BUFFER_LENGTH + 1); // +1 so I can add first, and then check if len() > MAX_BUFFER_LENGTH

    loop {
        let mut byte_buf = [0 as u8; 1];
        stream.read_exact(&mut byte_buf)?;

        buf.push(byte_buf[0]);

        //protocol specifies '\r\n' as end of line
        if buf.len() >= 2 && buf[buf.len() - 2] == b'\r' && buf[buf.len() - 1] == b'\n' {

            let line_str: String = match String::from_utf8(buf[..buf.len() - 2].to_vec()) {
                Err(_) => {
                    return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_NOT_UTF8_STRING))
                }
                Ok(s) => s,
            };

            res.push(line_str);
            buf = Vec::new();

            if res.last().unwrap().starts_with("SSH-2.0-") {
                break;
            } else {
                if !allow_comments {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          errors::BSSH_ERR_EXPECTED_HEADER_STRING));
                };

                if res.len() > MAX_COMMENT_LINES {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          errors::BSSH_ERR_TOO_MANY_COMMENT_LINES));
                };
            };

        } else {
            if buf.len() > MAX_BUFFER_LENGTH {
                return Err(Error::new(ErrorKind::InvalidData,
                                      errors::BSSH_ERR_NO_LINE_TERMINATION_FOUND));
            }
        };
    }

    Ok(res)
}

pub fn write_kex_init_message(stream: &mut Write,
                              config: &config::AvailableAlgorithms,
							  first_kex_packet_follows : bool)
                              -> Result<(), Error> {
    stream.write(&[numbers::SSH_MSG_KEXINIT])?;

    let cookie: [u8; 16] = rand::random::<[u8; 16]>();
    stream.write(&cookie)?;

    let kex_algorithms = config.get_available_kex_algorithms();
    io_helpers::write_name_list(stream, &kex_algorithms)?;

    let server_host_key_algorithms = config.get_available_server_host_key_algorithms();
    io_helpers::write_name_list(stream, &server_host_key_algorithms)?;

    let encryption_algorithms_client_to_server = config.get_available_encryption_algorithms_client_to_server();
    io_helpers::write_name_list(stream, &encryption_algorithms_client_to_server)?;
    let encryption_algorithms_server_to_client = config.get_available_compression_algorithms_server_to_client();
    io_helpers::write_name_list(stream, &encryption_algorithms_server_to_client)?;

    let mac_algorithms_client_to_server = config.get_available_mac_algorithms_client_to_server();
    io_helpers::write_name_list(stream, &mac_algorithms_client_to_server)?;
    
    let mac_algorithms_server_to_client = config.get_available_mac_algorithms_server_to_client();
    io_helpers::write_name_list(stream, &mac_algorithms_server_to_client)?;

	let compression_algorithms_client_to_server = config.get_available_compression_algorithms_client_to_server();
	io_helpers::write_name_list(stream, &compression_algorithms_client_to_server)?;
	
	let compression_algorithms_server_to_client = config.get_available_compression_algorithms_server_to_client();
	io_helpers::write_name_list(stream, &compression_algorithms_server_to_client)?;
	
	let languages_client_to_server = config.get_available_languages_client_to_server();
	io_helpers::write_name_list(stream, &languages_client_to_server)?;
	
	let languages_server_to_client = config.get_available_languages_server_to_client();
	io_helpers::write_name_list(stream, &languages_server_to_client)?;
	
	
	
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::read_welcome_string;
    use std::io::*;
    use errors;

    struct MockStream {
        input: Vec<u8>,
        pos: usize,
    }

    impl Read for MockStream {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            if buf.len() > (self.input.len() - self.pos) {
                Err(Error::new(ErrorKind::BrokenPipe, "")) //TODO ok errorkind?
            } else {
                for i in 0..buf.len() {
                    buf[i] = self.input[self.pos + i];
                }
                self.pos += buf.len();
                Ok(())
            }
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            panic!();
        }
    }

    #[test]
    fn read_welcome_string_accepts_simple_string() {
        let input = b"SSH-2.0-hello-world\r\n".to_vec();
        let mut ms = MockStream {
            input: input,
            pos: 0,
        };

        assert_eq!(read_welcome_string(&mut ms, false).unwrap(),
                   vec!["SSH-2.0-hello-world".to_string()]);
        ms.pos = 0;
        assert_eq!(read_welcome_string(&mut ms, true).unwrap(),
                   vec!["SSH-2.0-hello-world".to_string()]);
    }

    #[test]
    fn read_welcome_string_accepts_commentary_when_asked() {
        let input = b"Hello\r\nWorld\r\nSSH-2.0-hello-world\r\n".to_vec();
        let mut ms = MockStream {
            input: input,
            pos: 0,
        };

        assert!(read_welcome_string(&mut ms, false).is_err());
        ms.pos = 0;
        assert_eq!(read_welcome_string(&mut ms, true).unwrap(),
                   vec!["Hello".to_string(),
                        "World".to_string(),
                        "SSH-2.0-hello-world".to_string()]);
    }

    struct MockStreamInfitnite {}

    impl Read for MockStreamInfitnite {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            for i in 0..buf.len() {
                buf[i] = b'.';
            }
            Ok(())
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            panic!();
        }
    }

    #[test]
    fn read_welcome_string_handles_overflow() {
        let mut msi = MockStreamInfitnite {};
        assert!(read_welcome_string(&mut msi, false).is_err());
        assert!(read_welcome_string(&mut msi, true).is_err());
    }

    struct MockStreamInfitniteComment {
        pos: u8,
    }

    impl Read for MockStreamInfitniteComment {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            for i in 0..buf.len() {
                buf[i] = match self.pos {
                    0 => b'.',
                    1 => b'\r',
                    2 => b'\n',
                    _ => panic!(),
                };
                self.pos += 1;
                self.pos %= 3;
            }
            Ok(())
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            panic!();
        }
    }

    #[test]
    fn read_welcome_string_handles_comment_overflow() {
        let mut msic = MockStreamInfitniteComment { pos: 0 };
        assert!(read_welcome_string(&mut msic, false).is_err());
        msic.pos = 0;
        assert!(read_welcome_string(&mut msic, true).is_err());
    }

}
