use std::error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Cursor;

extern crate bsshlib;
extern crate num;
extern crate rand; //TODO replace with some safe rand someday

use bsshlib::version;
use bsshlib::msgs;
use bsshlib::dummy_config;
use bsshlib::config;
use bsshlib::packet;
use bsshlib::diffie_hellman;
use num::bigint::{BigUint, RandBigInt, ToBigUint};

const HOST: &'static str = "127.0.0.1:22";

fn connect() -> Result<(), Box<error::Error + Send + Sync>> {

	let mut sequence_number : u32 = 0;
    let hello: Vec<u8> = [version::get_version_byte_string(), b"\r\n".to_vec()].concat();

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&hello));

    let welcome : Vec<String> = msgs::read_welcome_string(&mut stream, true)?;

    for i in welcome.iter() { println!("{}",i); };

	let config = dummy_config::DummyCommonConfig{};
	let kex = msgs::create_kex_init_message(&config, false);
	
	let mut kex_payload : Vec<u8> = Vec::new();
	msgs::write_kex_init_message(&mut kex_payload, &kex)?;
	let mut kex_message : Vec<u8> = packet::get_packet_from_payload(&mut kex_payload, None, false, sequence_number)?;
	stream.write(&mut kex_message)?;
	
	sequence_number += 1;
	
	let ret_kex_payload : Vec<u8> = packet::read_packet_from_stream(&mut stream, None)?;
	let mut x = Cursor::new(ret_kex_payload);
	let ret_kex_message = msgs::read_kex_init_message(&mut x)?;
	
	println!("{}", &ret_kex_message.available_algorithm_set as &config::AvailableAlgorithms);

	//diffie-hellman
	let mut rng = rand::thread_rng();
	let p : BigUint = diffie_hellman::get_oakley_group14_prime();
	let x : BigUint = rng.gen_biguint_range(&1.to_biguint().unwrap(), &p);
	let e : BigUint = num::pow(x, 2) % p;

	let mut kexdh_payload : Vec<u8> = Vec::new();	
	msgs::write_kexdh_init_message(&mut kexdh_payload, e)?;
	let kexdh_payload_len = kexdh_payload.len();
	let mut kexdh_message : Vec<u8> = packet::get_packet_from_payload(&mut kexdh_payload, None, true, sequence_number)?;
	println!("sending kexdh len {} payload {}", kexdh_message.len(), kexdh_payload_len);
	stream.write(&mut kexdh_message)?;
	
	sequence_number += 1;
	
	let mut buf;
    loop {
    	let mut done = false;
        // clear out the buffer so we don't send garbage
        buf = [0; 4*1024];
        let num = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    println!("eof");
                    done = true;
                }
                m
            }
        };

        let mut vbuf = buf.to_vec();
        vbuf.truncate(num);
        println!("got {:?}: {:?}", num, vbuf);
        if done { break; };
    }

    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() {

    match connect() {
        Ok(_) => println!("ok"),
        Err(err) => println!("An error occurred: {}", err),
    }
}
