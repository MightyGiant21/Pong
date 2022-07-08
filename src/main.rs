mod client;
mod keyboard_manager;
mod player_manager;
mod server;
mod window_manager;
mod tests;

use std::{thread, sync::mpsc};

use keyboard_manager::KeyStroke;
use player_manager::{Players, Direction};
use server::GameServer;
use window_manager::WindowManager;

fn main() {
    let addr = String::from("0.0.0.0:3333");
    let mut send_addr = String::new();
    let mut window = WindowManager::build_window();
    let mut players = Players::init_players(&window.window_size);

    let mut which_player = 0;
    // // This automatically changes the addr for each player if the port is already busy
    let mut server = GameServer::start_server(&addr);

    // // This needs to write to the opposite addr
    match server.listener.local_addr() {
        Ok(addr) => {
            match addr.to_string().as_str() {
                "0.0.0.0:3333" => {send_addr = String::from("0.0.0.0:3334");
                                    which_player = 0},
                "0.0.0.0:3334" => {send_addr = String::from("0.0.0.0:3333");
                                    which_player = 1},
                _ => {}
            }   
        },
        Err(_) => send_addr = String::from("0.0.0.0::8000") 
    }

    let (tx, rx) = mpsc::channel::<Direction>();

    thread::spawn(move || {
        let tx1 = tx.clone();
        server.listen_to_stream(tx1)
    });

    'running: loop {
        match rx.try_recv() {
            Ok(dir) => {
                let other_player = match which_player {
                    0 => 1,
                    1 => 0,
                    _ => 0
                };
                players.player[other_player].move_paddle(dir)
            },
            Err(_) => {}
        };

        match keyboard_manager::get_key_press(&mut window.event_pump) {
            KeyStroke::EndGame => break 'running,
            KeyStroke::Up => {
                let msg = String::from("up");
                client::write_to_stream(&msg, &send_addr);
                players.player[which_player].move_paddle(Direction::Up)
            }
            KeyStroke::Down => {
                let msg = String::from("down");
                client::write_to_stream(&msg, &send_addr);
                players.player[which_player].move_paddle(Direction::Down)
            }
            KeyStroke::None => {}
        }
        window.refresh(&mut players);
    }

}
