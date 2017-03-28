use std::error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;

extern crate bsshlib;

use bsshlib::version;
use bsshlib::msgs;
use bsshlib::dummy_config;
use bsshlib::config;


const HOST: &'static str = "127.0.0.1:5555";

fn connect() -> Result<(), Box<error::Error + Send + Sync>> {

    let hello: Vec<u8> = [version::get_version_byte_string(), b"\r\n".to_vec()].concat();

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&hello));

    let welcome : Vec<String> = msgs::read_welcome_string(&mut stream, true)?;
    
    for i in welcome.iter() { println!("{}",i); };
    
	let config = dummy_config::DummyCommonConfig{};
	
	msgs::write_kex_init_message(&mut stream, &config, false)?;
	
	let kex_message = msgs::read_kex_init_message(&mut stream)?;
	
	println!("{}", &kex_message.available_algorithm_set as &config::AvailableAlgorithms);

    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() {

    match connect() {
        Ok(_) => println!("ok"),
        Err(err) => println!("An error occurred: {}", err),
    }
}
