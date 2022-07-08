use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender
};

use crate::player_manager::Direction;

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

    pub fn listen_to_stream(&mut self, tx: Sender<Direction>) {
        for stream in self.listener.incoming() {
            let mut stream = get_stream(stream);
            let mut buf = [0; 255];

            match stream.read(&mut buf) {
                Ok(_) => {
                    let msg = String::from_utf8(buf.to_vec()).unwrap();
                    let dir = decode_message(msg); 
                    tx.send(dir).unwrap();
                }
                Err(_) => {}
            };
        }
    }
}

fn decode_message(msg: String) -> Direction {
    let msg_v = msg.chars().collect::<Vec<char>>();
    let mut decoded_msg = String::new();

    for i in 0..msg.len() {
        if msg.as_bytes()[i] != 0 {
            decoded_msg.push(msg_v[i]) 
        }
    }

    match decoded_msg.as_str() {
        "up" => return Direction::Up,
        "down" => return Direction::Down,
        _ => return Direction::None
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
