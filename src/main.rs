mod client;
mod keyboard_manager;
mod player_manager;
mod server;
mod window_manager;

use std::{thread, sync::mpsc};

use keyboard_manager::KeyStroke;
use player_manager::Players;
use server::GameServer;
use window_manager::WindowManager;

fn main() {
    let addr = String::from("0.0.0.0:3333");
    let mut send_addr = String::new();

    // This automatically changes the addr for each player if the port is already busy
    let mut server = GameServer::start_server(&addr);

    // This needs to write to the opposite addr
    match server.listener.local_addr() {
        Ok(addr) => {
            match addr.to_string().as_str() {
                "0.0.0.0:3333" => send_addr = String::from("0.0.0.0:3334"),
                "0.0.0.0:3334" => send_addr = String::from("0.0.0.0:3333"),
                _ => {}
            }   
        },
        Err(_) => send_addr = String::from("0.0.0.0::8000") 
    }
    
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        loop {
        // println!("{}", rx.recv().unwrap());
        }
    });

    thread::spawn(move || {
        server.listen_to_stream();
    });

    let mut msg;


    let mut window = WindowManager::build_window();
    let mut players = Players::init_players(&window.window_size);

    'running: loop {
        match keyboard_manager::get_key_press(&mut window.event_pump) {
            KeyStroke::EndGame => break 'running,
            KeyStroke::Up => {
                msg = "up".to_string();
                client::write_to_stream(&msg, &send_addr);
                let tx1 = tx.clone();
                thread::spawn(move || {
                    tx1.send(msg).unwrap();
                });
                players.player[0].move_paddle(KeyStroke::Up)
            }
            KeyStroke::Down => {
                msg = "down".to_string();
                client::write_to_stream(&msg, &send_addr);
                let tx1 = tx.clone();
                thread::spawn(move || {
                    tx1.send(msg).unwrap();
                });
                players.player[0].move_paddle(KeyStroke::Down)
            }
            KeyStroke::None => {}
        }
        window.refresh(&mut players);
    }
}
