use std::io::{Cursor, Read, Write};
use std::io::{Error, ErrorKind};
use std::mem::size_of;
use std::str::from_utf8;
use errors;

use num::bigint::{BigInt, BigUint, ToBigInt, Sign};

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

    for i in 0..names.len() {
        payload.extend_from_slice(names[i].as_bytes());
        if i < names.len() - 1 {
            payload.push(b',');
        }
    }

    write_string(stream, &payload)
}

pub fn read_name_list(stream: &mut Read, max_length: Option<u32>) -> Result<Vec<String>, Error> {
    let payload: Vec<u8> = read_string(stream, max_length)?;
    let mut res: Vec<String> = Vec::new();

    if payload.len() == 0 {
        return Ok(res);
    }

    for substr in payload.split(|c| *c == b',') {
        //we expect the substrings to be non-empty
        if substr.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_MALFORMED_NAME_LIST));
        }
        match from_utf8(&substr) {
            Ok(s) => res.push(s.to_string()),
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_MALFORMED_NAME_LIST))
            } //TODO more descriptive error?
        }
    }

    Ok(res)
}

pub fn write_boolean(stream: &mut Write, value: bool) -> Result<(), Error> {
    let payload: [u8; 1] = [if value { 1 as u8 } else { 0 as u8 }; 1];
    stream.write_all(&payload)?;
    Ok(())
}

pub fn read_boolean(stream: &mut Read) -> Result<bool, Error> {
    let mut payload: [u8; 1] = [0; 1];
    stream.read_exact(&mut payload)?;
    if payload[0] == 0 { Ok(false) } else { Ok(true) }
}

pub fn write_mpint(stream: &mut Write, value: BigInt) -> Result<(), Error> {
    if value == 0.to_bigint().unwrap() {
        stream.write(&[0 as u8; 4])?;
        Ok(())
    } else {
        let (sign, mut tail) = value.to_bytes_be();

        /* RFC4251: If the most significant bit would be set for a positivce number, the number MUST be preceded by a zero byte. */
        /* own comment: If the most significant bit in negative number is set, we need to preceed by a 80 (hex) byte*/
        /* bottom line, no matter the sign, most significant byte set => one additional byte needed */
        let most_significant_byte_positive: bool = tail[0] & (128 as u8) == (128 as u8);
        let length = tail.len() + (if most_significant_byte_positive { 1 } else { 0 });

        //first write the length as u32, BigEndian
        let mut length_bytes: Vec<u8> = Vec::new();
        length_bytes.write_u32::<BigEndian>(length as u32)?;
        stream.write_all(&length_bytes)?;

        if most_significant_byte_positive {
            if sign == Sign::Plus {
                stream.write(&[0 as u8])?;
            } else {
                stream.write(&[0xff as u8])?;
            }
        }

        if sign == Sign::Minus {
            for i in 0..tail.len() {
                tail[i] ^= 255 as u8;
            }
            let tail_length = tail.len();
            tail[tail_length - 1] += 1;
        }

        stream.write_all(&tail)?;
        Ok(())
    }
}

pub fn read_mpint(stream: &mut Read) -> Result<BigInt, Error> {
    let mut length_bytes: Vec<u8> = Vec::new();
    length_bytes.resize(size_of::<u32>(), 0);
    stream.read_exact(&mut length_bytes)?;
    let length = Cursor::new(length_bytes)
        .read_u32::<BigEndian>()
        .unwrap();

    if length == 0 {
        return Ok(0.to_bigint().unwrap());
    }

    let mut body: Vec<u8> = Vec::new();
    body.resize(length as usize, 0);
    stream.read_exact(&mut body)?;

    let sign = if body[0] & (128 as u8) == (128 as u8) {
        Sign::Minus
    } else {
        Sign::Plus
    };

    if sign == Sign::Minus {
        for i in 0..body.len() {
            body[i] ^= 255 as u8;
        }
        let body_length = body.len();
        body[body_length - 1] += 1;
    }

	let res = BigInt::from_bytes_be(sign, &body);
	
	Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::*;
    use mocks::*;

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
        let hello_string: Vec<u8> = [[0 as u8, 0, 0, 11].to_vec(), b"hello,world".to_vec()]
            .concat();
        let mut mrs = MockReadStream::new(hello_string);
        assert_eq!(read_name_list(&mut mrs, None).unwrap(),
                   vec!["hello", "world"]);
    }

    #[test]
    fn read_name_lists_tolerates_empty_list() {
        let empty_list_string: Vec<u8> = [0 as u8, 0, 0, 0].to_vec();
        let mut mrs = MockReadStream::new(empty_list_string);
        assert_eq!(read_name_list(&mut mrs, None).unwrap().len(), 0);
    }

    #[test]
    fn write_name_list_works() {
        let hello_string: Vec<u8> = [[0 as u8, 0, 0, 11].to_vec(), b"hello,world".to_vec()]
            .concat();
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

    #[test]
    fn write_mpint_works() {
        //these tests are copied examples from RFC4251, page 10
        {
            let mut mws = MockWriteStream::new();
            write_mpint(&mut mws, 0.to_bigint().unwrap()).unwrap();
            assert_eq!(mws.output, vec![0 as u8; 4]);
        }

        {
            let mut mws = MockWriteStream::new();
            let biguint: BigUint = BigUint::parse_bytes(b"9a378f9b2e332a7", 16).unwrap();
            write_mpint(&mut mws, biguint.to_bigint().unwrap()).unwrap();
            assert_eq!(mws.output,
                       vec![0 as u8, 0, 0, 0x08, 0x09, 0xa3, 0x78, 0xf9, 0xb2, 0xe3, 0x32, 0xa7]);
        }

        {
            let mut mws = MockWriteStream::new();
            let biguint: BigUint = BigUint::parse_bytes(b"80", 16).unwrap();
            write_mpint(&mut mws, biguint.to_bigint().unwrap()).unwrap();
            assert_eq!(mws.output, vec![0 as u8, 0, 0, 0x02, 0x00, 0x80]);
        }

        {
            let mut mws = MockWriteStream::new();
            let bigint: BigInt = BigInt::parse_bytes(b"-1234", 16).unwrap();
            println!("{}", bigint);
            let (sign, tail) = bigint.to_bytes_be();
            println!("{:?}", tail);
            write_mpint(&mut mws, bigint).unwrap();
            assert_eq!(mws.output, vec![0 as u8, 0, 0, 0x02, 0xed, 0xcc]);
        }

        {
            let mut mws = MockWriteStream::new();
            let bigint: BigInt = BigInt::parse_bytes(b"-deadbeef", 16).unwrap();
            write_mpint(&mut mws, bigint).unwrap();
            assert_eq!(mws.output,
                       vec![0 as u8, 0, 0, 0x05, 0xff, 0x21, 0x52, 0x41, 0x11]);
        }
    }
    
    #[test]
	fn read_mpint_works() {
        //these tests are copied examples from RFC4251, page 10
        {
            let mut mrs = MockReadStream::new(vec![0 as u8; 4]);
            let bigint = read_mpint(&mut mrs).unwrap();
            assert_eq!(bigint, BigInt::parse_bytes(b"0", 16).unwrap())
        }

        {
            let mut mrs = MockReadStream::new(vec![0 as u8, 0, 0, 0x08, 0x09, 0xa3, 0x78, 0xf9, 0xb2, 0xe3, 0x32, 0xa7]);
            let bigint = read_mpint(&mut mrs).unwrap();
            assert_eq!(bigint, BigInt::parse_bytes(b"9a378f9b2e332a7", 16).unwrap())
        }
        
        {
            let mut mrs = MockReadStream::new(vec![0 as u8, 0, 0, 0x02, 0xed, 0xcc]);
            let bigint = read_mpint(&mut mrs).unwrap();
            assert_eq!(bigint, BigInt::parse_bytes(b"-1234", 16).unwrap())
        }
        
        {
            let mut mrs = MockReadStream::new(vec![0 as u8, 0, 0, 0x05, 0xff, 0x21, 0x52, 0x41, 0x11]);
            let bigint = read_mpint(&mut mrs).unwrap();
            assert_eq!(bigint, BigInt::parse_bytes(b"-deadbeef", 16).unwrap())
        }
    }
}
