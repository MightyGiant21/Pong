use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
    str::from_utf8,
};

pub struct GameServer {
    listener: TcpListener,
}

impl GameServer {
    pub fn start_server(addr: String) -> GameServer {
        let listener = create_listener(addr);

        GameServer { listener }
    }

    pub fn listen_to_stream(&mut self) {
        for stream in self.listener.incoming() {
            let mut stream = get_stream(stream);
            let mut buf = [0; 512];

            match stream.read(&mut buf) {
                Ok(_) => {
                    println!("data {}", from_utf8(&mut buf).unwrap());
                }
                Err(_) => {}
            };

            // handle_connection(stream)
        }
    }
}

fn get_stream(stream_res: Result<TcpStream, Error>) -> TcpStream {
    let stream;

    match stream_res {
        Ok(s) => stream = s,
        Err(e) => panic!("Failed to listen to stream due to: {}", e),
    };

    stream
}

fn create_listener(addr: String) -> TcpListener {
    let listener_attempt = TcpListener::bind(addr);
    let listener;

    match listener_attempt {
        Ok(l) => listener = l,
        Err(e) => {
            panic!("Failed to bind due to: {}", e)
        }
    }

    listener
}
