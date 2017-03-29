use std::mem;
use std::io::{Cursor, Error, ErrorKind, Read};
use rand;
use errors;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

const MAX_PACKET_LENGTH : usize = 4*1024*1024; //TODO arbitrary value, probably too large

pub fn get_packet_from_payload(payload: &mut Vec<u8>, cipher_block_size : Option<u8>) -> Result<Vec<u8>, Error> {
    let mut result: Vec<u8> = Vec::new();

    let packet_length_without_random = 1 + payload.len();
    //TODO this is ad-hoc formula
    let alignment : u8 = match cipher_block_size {
    	Some(size) => size,
    	None => 8 //rfc4253 on random padding
    };
    
    let min_padding_length: u8 = alignment - ((packet_length_without_random % alignment as usize) as u8);

	//RFC 4253 page 8 "there MUST be at least four bytes of padding"
	let padding_length = if min_padding_length >= 4 { min_padding_length } else {min_padding_length + alignment};

    let mut random_padding: Vec<u8> = Vec::new();
    random_padding.reserve(padding_length as usize);
    for _ in 0..padding_length {
        random_padding.push(rand::random::<u8>());
    }

	//packet_length - not including 'mac' or the 'packet_length' itself.
	let packet_length : usize = 1 + payload.len() + random_padding.len();
	println!("writing packet_length {}", packet_length);
    result.write_u32::<BigEndian>(packet_length as u32)?;
    result.push(padding_length);
    println!("writing padding_length {}", padding_length);
    result.append(payload);
    result.append(&mut random_padding);

    //TODO MAC

    Ok(result)
}

pub fn read_packet_from_stream(stream : &mut Read, cipher_block_size : Option<u8>) -> Result<Vec<u8>, Error> {
	let read_to_determine_packet_length : u8 = match cipher_block_size {
    	Some(size) => size,
    	None => 4
    };
	
	let mut length_buffer : Vec<u8> = Vec::new();
	length_buffer.resize(read_to_determine_packet_length as usize, 0 as u8);
	stream.read_exact(&mut length_buffer)?;
	
	//TODO here some decoding takes place?
	let packet_length : u32 = Cursor::new(length_buffer[0..4].to_vec()).read_u32::<BigEndian>()?; //TODO this is horrible
	println!("reading packet_length = {}", packet_length);
	
	if packet_length as usize > MAX_PACKET_LENGTH {
		return Err(Error::new(ErrorKind::InvalidData, errors::BSSH_ERR_BUFFER_CAPACITY_EXCEEDED)); //TODO better message?
	}
	
	//TODO if there is a remainder of what has been read except the length_buffer, this needs to be transfered here
	
	//read padding_length
	let mut padding_length_buf : [u8; 1] = [0; 1];
	stream.read(&mut padding_length_buf)?;
	let padding_length : u8 = padding_length_buf[0];
	println!("reading padding_length = {}", padding_length);
	
	let mut buffer : Vec<u8> = Vec::new();
	buffer.resize((packet_length - 1) as usize, 0); // -1 stands for padding_length which is read before
	stream.read_exact(&mut buffer)?;
	println!("read!");
	println!("{:?}", &buffer);
	Ok(buffer[0..(buffer.len() - (padding_length as usize))].to_vec()) //TODO this is also horrible
}