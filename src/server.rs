use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
    str::from_utf8, sync::mpsc, thread,
};

pub struct GameServer {
    listener: TcpListener,
    pub msg: String
}

impl GameServer {
    pub fn start_server(addr: String) -> GameServer {
        let listener = create_listener(addr);
        let msg = String::new();
        GameServer { listener, msg}
    }

    pub fn listen_to_strea(&mut self) {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let msg = String::from("Hi");
            tx.send(msg).unwrap();
        });

    }

    pub fn listen_to_stream(&mut self) {
        for stream in self.listener.incoming() {
            let mut stream = get_stream(stream);
            let mut buf = [0; 512];

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
