use std::error;
use std::io::Write;
use std::io::Read;
use std::io::Cursor;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::net::Shutdown;
use std::thread;

extern crate bsshlib;
use bsshlib::version;
use bsshlib::msgs;
use bsshlib::dummy_config;
use bsshlib::config;
use bsshlib::packet;

const HOST: &'static str = "127.0.0.1:5555";

fn handle_client(mut stream: TcpStream) -> Result<(), Box<error::Error + Send + Sync>> {
    let hello: Vec<u8> = [version::get_version_byte_string(), b"\r\n".to_vec()].concat();
    try!(stream.write_all(&hello));

    let welcome : Vec<String> = msgs::read_welcome_string(&mut stream, false)?;
    
    for i in welcome.iter() { println!("{}",i); };
    
	let config = dummy_config::DummyCommonConfig{};
	let kex = msgs::create_kex_init_message(&config, false);
	
	let mut kex_payload : Vec<u8> = Vec::new();
	msgs::write_kex_init_message(&mut kex_payload, &kex)?;
	let mut kex_message : Vec<u8> = packet::get_packet_from_payload(&mut kex_payload, None)?;
	stream.write(&mut kex_message)?;
	
	let ret_kex_payload : Vec<u8> = packet::read_packet_from_stream(&mut stream, None)?;
	let mut x = Cursor::new(ret_kex_payload);
	let ret_kex_message = msgs::read_kex_init_message(&mut x)?;
	
	println!("{}", &ret_kex_message.available_algorithm_set as &config::AvailableAlgorithms);

    let mut buf;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        let num = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    println!("eof");
                    break;
                }
                m
            }
        };

        let mut vbuf = buf.to_vec();
        vbuf.truncate(num);
        println!("got {:?}: {:?}", num, vbuf);

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                		match handle_client(stream) {
                			Err(e) => println!("failed2: {}", e),
                			_ => {}
                		}
                });
            }
        }
    }
}
