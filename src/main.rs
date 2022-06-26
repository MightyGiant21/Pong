mod window_manager;
mod keyboard_manager;
mod player_manager;

use keyboard_manager::KeyStroke;
use player_manager::Players;
use window_manager::WindowManager;


fn main() {
    let mut window = WindowManager::build_window();
    let mut players = Players::init_players(&window.window_size);

    'running: loop {
        if keyboard_manager::get_key_press(&mut window.event_pump) == KeyStroke::EndGame {
            break 'running
        }
        window.refresh(&mut players);
    }
}   
