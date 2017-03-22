use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;
use std::thread;

mod version;

const HOST: &'static str = "127.0.0.1:5555";

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {

        let hello : Vec<u8> = [version::get_version_byte_string(),  b"\r\n".to_vec()].concat();

        stream.write_all(&hello);

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
            },
        };

        let mut vbuf = buf.to_vec();
        vbuf.truncate(num);
        println!("got {:?}: {:?}", num, vbuf);

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
