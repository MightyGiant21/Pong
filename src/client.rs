use std::{io::Write, net::TcpStream};

pub fn write_to_stream(msg: &str, addr: &str) {
    let mut stream = TcpStream::connect(addr).unwrap();

    stream.write(&msg.as_bytes()).unwrap();
    stream.flush().unwrap();
}
