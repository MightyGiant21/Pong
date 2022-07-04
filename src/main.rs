mod client;
mod keyboard_manager;
mod player_manager;
mod server;
mod window_manager;

use std::thread;

use keyboard_manager::KeyStroke;
use player_manager::Players;
use server::GameServer;
use window_manager::WindowManager;

fn main() {
    let addr = String::from("0.0.0.0:3333");

    thread::spawn(move || {
        let mut server = GameServer::start_server(addr);
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
                client::write_to_stream(&msg);
                players.player[0].move_paddle(KeyStroke::Up)
            }
            KeyStroke::Down => {
                msg = "down".to_string();
                client::write_to_stream(&msg);
                players.player[0].move_paddle(KeyStroke::Down)
            }
            KeyStroke::None => {}
        }
        window.refresh(&mut players);
    }
}
