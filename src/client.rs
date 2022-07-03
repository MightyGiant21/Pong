use std::{io::Write, net::TcpStream};

pub fn write_to_stream(msg: &str) {
    let addr = String::from("localhost:3333");
    let mut stream = TcpStream::connect(addr).unwrap();

    stream.write(&msg.as_bytes()).unwrap();
}
