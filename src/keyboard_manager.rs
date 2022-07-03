use sdl2::{event::Event, keyboard::Keycode, EventPump};

#[derive(PartialEq)]
pub enum KeyStroke {
    Up,
    Down,
    None,
    EndGame,
}

pub fn get_key_press(event_pump: &mut EventPump) -> KeyStroke {
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return KeyStroke::EndGame,
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => return KeyStroke::Up,
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => return KeyStroke::Down,
            _ => return KeyStroke::None,
        }
    }
    KeyStroke::None
}
