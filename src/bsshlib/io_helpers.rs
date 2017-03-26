use std::io::{Cursor, Read, Write};
use std::io::{Error, ErrorKind};
use std::mem::size_of;

use errors;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

const MAX_BUFFER_LENGTH : u32 = 4*1024*1024; //TODO arbitrary value

pub fn write_string(stream : &mut Write, string : &Vec<u8>) -> Result<(), Error> {
    let mut length_bytes : Vec<u8> = Vec::new();
    length_bytes.write_u32::<BigEndian>(string.len() as u32);
    stream.write_all(&length_bytes)?;
    stream.write_all(string)?;

    Ok(())
}

pub fn read_string(stream : &mut Read, max_length : Option<u32>) -> Result<Vec<u8>, Error> {
    let mut length_bytes : Vec<u8> = Vec::new();
    length_bytes.resize(size_of::<u32>(), 0);
    stream.read_exact(&mut length_bytes)?;
    let length = Cursor::new(length_bytes).read_u32::<BigEndian>().unwrap();

    let actual_max_length : u32 = match max_length {
        Some(x) => x,
        None => MAX_BUFFER_LENGTH
    };

    if length > actual_max_length {
        return Err(Error::new(ErrorKind::Other, errors::BSSH_ERR_BUFFER_CAPACITY_EXCEEDED));
    }

    let mut body : Vec<u8> = Vec::new();
    body.resize(length as usize, 0);
    stream.read_exact(&mut body)?;

    Ok(body)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::*;
    use errors;
    use std::cmp::PartialEq;

    struct MockReadStream {
        input : Vec<u8>,
        pos : usize
    }

    impl MockReadStream {
        fn new(input : Vec<u8>) -> MockReadStream {
            MockReadStream { input : input, pos : 0 }
        }
    }

    impl Read for MockReadStream {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            if buf.len() > (self.input.len() - self.pos) {
                Err(Error::new(ErrorKind::BrokenPipe, "")) //TODO ok errorkind?
            } else {
                for i in 0..buf.len() {
                    buf[i] = self.input[self.pos + i];
                };
                self.pos += buf.len();
                Ok(())
            }
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> { panic!(); }
    }

    struct MockWriteStream {
        output : Vec<u8>
    }

    impl MockWriteStream {
        fn new() -> MockWriteStream {
            MockWriteStream { output : Vec::new() }
        }
    }

    impl Write for MockWriteStream {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.output.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn read_string_reads_string() {
        let hello_string : Vec<u8> = vec![0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o'];
        let mut mrs = MockReadStream::new(hello_string);
        assert_eq!(read_string(&mut mrs, None).unwrap(), b"hello".to_vec());

    }

    #[test]
    fn write_string_writes_string() {
        let mut mws = MockWriteStream::new();
        let hello_string : Vec<u8> = vec![0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o'];
        write_string(&mut mws, &b"hello".to_vec()).unwrap();
        assert_eq!(mws.output, hello_string);
    }

    #[test]
    fn read_string_does_not_overflow() {
        let hello_string : Vec<u8> = vec![255, 255, 255, 255, b'h', b'e', b'l', b'l', b'o'];
        let mut mrs = MockReadStream::new(hello_string);
        assert!(read_string(&mut mrs, None).is_err());

    }
}
