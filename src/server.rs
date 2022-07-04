use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
    str::from_utf8
};

pub struct GameServer {
    pub listener: TcpListener,
    pub msg: String
}

impl GameServer {
    pub fn start_server(addr: &str) -> GameServer {
        let listener = create_listener(addr);
        let msg = String::new();
        GameServer { listener, msg}
    }

    pub fn listen_to_stream(&mut self) {
        for stream in self.listener.incoming() {
            let mut stream = get_stream(stream);
            let mut buf = [0; 512];

            println!("{:?}", stream);
            match stream.read(&mut buf) {
                Ok(_) => {
                    self.msg = from_utf8(&mut buf).unwrap().to_string();
                }
                Err(_) => {}
            };
        }
    }
}

fn get_stream(stream_res: Result<TcpStream, Error>) -> TcpStream {
    match stream_res {
        Ok(stream) => return stream,
        Err(e) => panic!("Failed to listen to stream due to: {}", e),
    };
}

fn create_listener(addr: &str) -> TcpListener {
    match TcpListener::bind(addr) {
        Ok(listener) => return listener,
        Err(_) => {
            return create_listener(&String::from("0.0.0.0:3334"));
        }
    }
}
