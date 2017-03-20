// use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut inp: Vec<u8> = Vec::new();
            stream.read_to_end(&mut inp);
            println!("{:?}", inp);
        });
    }
}
