use std::io::{Error, ErrorKind, Read, Write};
use rand;
use config;
use errors;
use numbers;
use io_helpers;

const MAX_BUFFER_LENGTH: usize = 255; //RFC 4253 page 5
const MAX_COMMENT_LINES: usize = 100; //TODO arbitrary value

pub struct KexMessage {
    pub cookie: [u8; 16],
    pub available_algorithm_set: config::AvailableAlgorithmSet,
    pub first_kex_packet_follows: bool,
}

impl KexMessage {
    fn get_cookie(&self) -> [u8; 16] {
        self.cookie
    }
    fn get_first_kex_packet_follows(&self) -> bool {
        self.first_kex_packet_follows
    }
}

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
	
	io_helpers::write_boolean(stream, first_kex_packet_follows)?;
	
	let empty_u32 : [u8; 4] = [0; 4];
	stream.write(&empty_u32)?;
	
    Ok(())
}
                              
pub fn read_kex_init_message(stream : &mut Read) -> Result<KexMessage, Error> {
	let mut init_byte : [u8; 1] = [0; 1];
	stream.read_exact(&mut init_byte)?;
	
	if init_byte[0] != numbers::SSH_MSG_KEXINIT {
		return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_EXPECTED_KEX_MSG_INIT));
	}
	
	let mut cookie : [u8; 16] = [0; 16];
	stream.read_exact(&mut cookie)?;
	
	let kex_algorithms = io_helpers::read_name_list(stream, None)?;
	let server_host_key_algorithms = io_helpers::read_name_list(stream, None)?;
	let encryption_algorithms_client_to_server = io_helpers::read_name_list(stream, None)?;
	let encryption_algorithms_server_to_client = io_helpers::read_name_list(stream, None)?;
	let mac_algorithms_client_to_server = io_helpers::read_name_list(stream, None)?;
	let mac_algorithms_server_to_client = io_helpers::read_name_list(stream, None)?;
	let compression_algorithms_client_to_server = io_helpers::read_name_list(stream, None)?;
	let compression_algorithms_server_to_client = io_helpers::read_name_list(stream, None)?;
	let languages_client_to_server = io_helpers::read_name_list(stream, None)?;
	let languages_server_to_client = io_helpers::read_name_list(stream, None)?;
	let first_kex_packet_follows : bool = io_helpers::read_boolean(stream)?;
	
	let mut empty_u32 : [u8; 4] = [0; 4];
	stream.read_exact(&mut empty_u32)?;
	if empty_u32[0] != 0 || empty_u32[1] != 0 || empty_u32[2] != 0 || empty_u32[3] != 0 {
		return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_EXPECTED_ZERO_U32));
	}
	
	let kex_message = KexMessage {
		cookie : cookie,
		available_algorithm_set : config::AvailableAlgorithmSet {
			kex_algorithms : kex_algorithms,
			server_host_key_algorithms : server_host_key_algorithms,
			encryption_algorithms_client_to_server : encryption_algorithms_client_to_server,
			encryption_algorithms_server_to_client : encryption_algorithms_server_to_client,
			mac_algorithms_client_to_server : mac_algorithms_client_to_server,
			mac_algorithms_server_to_client : mac_algorithms_server_to_client,
			compression_algorithms_client_to_server : compression_algorithms_client_to_server,
			compression_algorithms_server_to_client : compression_algorithms_server_to_client,
			languages_client_to_server : languages_client_to_server,
			languages_server_to_client : languages_server_to_client
		},
		first_kex_packet_follows : first_kex_packet_follows
	};
	Ok(kex_message)
}

#[cfg(test)]
mod tests {

    use super::read_welcome_string;
    use std::io::*;
    use errors;
    use mocks;

    #[test]
    fn read_welcome_string_accepts_simple_string() {
        let input = b"SSH-2.0-hello-world\r\n".to_vec();
        let mut mrs = mocks::MockReadStream::new(input);

        assert_eq!(read_welcome_string(&mut mrs, false).unwrap(),
                   vec!["SSH-2.0-hello-world".to_string()]);
        mrs.pos = 0;
        assert_eq!(read_welcome_string(&mut mrs, true).unwrap(),
                   vec!["SSH-2.0-hello-world".to_string()]);
    }

    #[test]
    fn read_welcome_string_accepts_commentary_when_asked() {
        let input = b"Hello\r\nWorld\r\nSSH-2.0-hello-world\r\n".to_vec();
        let mut mrs = mocks::MockReadStream::new(input);

        assert!(read_welcome_string(&mut mrs, false).is_err());
        mrs.pos = 0;
        assert_eq!(read_welcome_string(&mut mrs, true).unwrap(),
                   vec!["Hello".to_string(),
                        "World".to_string(),
                        "SSH-2.0-hello-world".to_string()]);
    }

    #[test]
    fn read_welcome_string_handles_overflow() {
        let mut msi = mocks::MockReadStreamInfitnite {};
        assert!(read_welcome_string(&mut msi, false).is_err());
        assert!(read_welcome_string(&mut msi, true).is_err());
    }

    struct MockReadStreamInfitniteComment {
        pos: u8,
    }

    impl Read for MockReadStreamInfitniteComment {
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
        let mut msic = MockReadStreamInfitniteComment { pos: 0 };
        assert!(read_welcome_string(&mut msic, false).is_err());
        msic.pos = 0;
        assert!(read_welcome_string(&mut msic, true).is_err());
    }

//	#[test]
//	fn reading_and_writing_kex_works() {
//		let mut 
//	}

}
