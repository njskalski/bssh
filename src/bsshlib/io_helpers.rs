use std::io::{Cursor, Read, Write};
use std::io::{Error, ErrorKind};
use std::mem::size_of;
use std::str::from_utf8;
use errors;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

const MAX_BUFFER_LENGTH: u32 = 4 * 1024 * 1024; //TODO arbitrary value

pub fn write_string(stream: &mut Write, string: &Vec<u8>) -> Result<(), Error> {
    let mut length_bytes: Vec<u8> = Vec::new();
    length_bytes.write_u32::<BigEndian>(string.len() as u32)?;
    stream.write_all(&length_bytes)?;
    stream.write_all(string)?;

    Ok(())
}

pub fn read_string(stream: &mut Read, max_length: Option<u32>) -> Result<Vec<u8>, Error> {
    let mut length_bytes: Vec<u8> = Vec::new();
    length_bytes.resize(size_of::<u32>(), 0);
    stream.read_exact(&mut length_bytes)?;
    let length = Cursor::new(length_bytes)
        .read_u32::<BigEndian>()
        .unwrap();

    let actual_max_length: u32 = match max_length {
        Some(x) => x,
        None => MAX_BUFFER_LENGTH,
    };

    if length > actual_max_length {
        return Err(Error::new(ErrorKind::Other, errors::BSSH_ERR_BUFFER_CAPACITY_EXCEEDED));
    }

    let mut body: Vec<u8> = Vec::new();
    body.resize(length as usize, 0);
    stream.read_exact(&mut body)?;

    Ok(body)
}

pub fn write_name_list(stream: &mut Write, names: &Vec<String>) -> Result<(), Error> {
    let mut payload: Vec<u8> = Vec::new();
    payload.push(b'(');

    for i in 0..names.len() {
        payload.push(b'"');
        payload.extend_from_slice(names[i].as_bytes());
        payload.push(b'"');
        if i < names.len() - 1 {
            payload.push(b',');
        }
    }

    payload.push(b')');
    write_string(stream, &payload)
}

pub fn read_name_list(stream: &mut Read, max_length: Option<u32>) -> Result<Vec<String>, Error> {
    let payload: Vec<u8> = read_string(stream, max_length)?;
    let mut res: Vec<String> = Vec::new();

    if payload.len() < 2 || payload[0] != b'(' || payload[payload.len() - 1] != b')' {
        return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_MALFORMED_NAME_LIST));
    }

    for substr in payload[1..payload.len() - 1].split(|c| *c == b',') {
        //we expect the substrings to be non-empty
        if substr.len() < 3 || substr[0] != b'\"' || substr[substr.len() - 1] != b'\"' {
            return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_MALFORMED_NAME_LIST));
        }
        let substr_body = substr[1..(substr.len() - 1)].to_vec();
        match from_utf8(&substr_body) {
            Ok(s) => res.push(s.to_string()),
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_MALFORMED_NAME_LIST))
            } //TODO more descriptive error?
        }
    }

    Ok(res)
}

pub fn write_boolean(stream : &mut Write, value : bool) -> Result<(), Error> {
	let payload : [u8; 1] = [if value { 1 as u8 } else { 0 as u8 }; 1];
	stream.write_all(&payload)?;
	Ok(())
}

pub fn read_boolean(stream : &mut Read) -> Result<bool, Error> {
	let mut payload : [u8; 1] = [0; 1];
	stream.read_exact(&mut payload)?;
	if payload[0] == 0 { Ok(false) } else { Ok(true) }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::*;	
    use tests::*;

    #[test]
    fn read_string_reads_string() {
        let hello_string: Vec<u8> = vec![0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o'];
        let mut mrs = MockReadStream::new(hello_string);
        assert_eq!(read_string(&mut mrs, None).unwrap(), b"hello".to_vec());

    }

    #[test]
    fn write_string_writes_string() {
        let mut mws = MockWriteStream::new();
        let hello_string: Vec<u8> = vec![0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o'];
        write_string(&mut mws, &b"hello".to_vec()).unwrap();
        assert_eq!(mws.output, hello_string);
    }

    #[test]
    fn read_string_does_not_overflow() {
        let hello_string: Vec<u8> = vec![255, 255, 255, 255, b'h', b'e', b'l', b'l', b'o'];
        let mut mrs = MockReadStream::new(hello_string);
        assert!(read_string(&mut mrs, None).is_err());

    }

    #[test]
    fn read_name_lists_works() {
        let hello_string: Vec<u8> =
            [[0 as u8, 0, 0, 17].to_vec(), b"(\"hello\",\"world\")".to_vec()].concat();
        let mut mrs = MockReadStream::new(hello_string);
        assert_eq!(read_name_list(&mut mrs, None).unwrap(),
                   vec!["hello", "world"]);
    }

    #[test]
    fn write_name_list_works() {
        let hello_string: Vec<u8> =
            [[0 as u8, 0, 0, 17].to_vec(), b"(\"hello\",\"world\")".to_vec()].concat();
        let mut mws = MockWriteStream::new();
        write_name_list(&mut mws, &vec!["hello".to_string(), "world".to_string()]).unwrap();
        assert_eq!(mws.output, hello_string);
    }
    
    #[test]
    fn read_boolean_works() {
    	//RFC 4251 page 9, "all non-zero calues MUST be intrpreted as TRUE"
    	let bools_string: Vec<u8> = [0 as u8, 1, 17].to_vec();
        let mut mrs = MockReadStream::new(bools_string);
        assert_eq!(read_boolean(&mut mrs).unwrap(), false);
        assert_eq!(read_boolean(&mut mrs).unwrap(), true);
        assert_eq!(read_boolean(&mut mrs).unwrap(), true);
    }
    
    #[test]
    fn write_boolean_works() {
        let mut mws = MockWriteStream::new();
        write_boolean(&mut mws, false).unwrap();
        write_boolean(&mut mws, true).unwrap();
        assert_eq!(mws.output, vec![0 as u8, 1]);
    }

}
