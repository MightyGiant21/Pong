mod window_manager;
mod keyboard_manager;

use keyboard_manager::KeyStroke;
use window_manager::WindowManager;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut window = WindowManager::build_window(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        if keyboard_manager::get_key_press(&mut event_pump) == KeyStroke::EndGame {
            break 'running
        }
        window.refresh();
    }
}   
