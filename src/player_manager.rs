use sdl2::rect::Rect;

use crate::keyboard_manager::KeyStroke;

pub struct Ball {
    pub rect: Rect,
}

impl Ball {
    pub fn new_ball(window_size: &(u32, u32), size: u32) -> Ball {
        let rect = Rect::new(
            window_size.0 as i32 / 2,
            window_size.1 as i32 / 2,
            size,
            size,
        );

        Ball { rect }
    }
}
pub struct Player {
    pub paddle: Rect,
    height: u32,
    top_coord: i32,
    bot_coord: i32,
}

impl Player {
    pub fn init_player(window_size: &(u32, u32)) -> Player {
        let x = 10;
        let width = window_size.0 / 70;
        let height = window_size.1 / 10;
        let top_coord = (window_size.1 as i32 / 2) + (height / 2) as i32;
        let paddle = Rect::new(x, top_coord, width, height);
        let bot_coord = top_coord + height as i32;

        Player {
            paddle,
            height,
            top_coord,
            bot_coord,
        }
    }

    pub fn move_paddle(&mut self, direction: KeyStroke) {
        // We need to change this so we can use the range when looking for a collision
        // Has nothing to do with renderring
        match direction {
            KeyStroke::Up => {
                self.top_coord -= 10;
                self.bot_coord -= 10;
            },
            KeyStroke::Down => {
                self.top_coord += 10;
                self.bot_coord += 10;
            },
            KeyStroke::None => {},
            KeyStroke::EndGame => {},
        }

        self.update_paddle_rect()
    }

    fn update_paddle_rect(&mut self) {
        self.paddle.y = self.top_coord;
    }
}

pub struct Players {
    pub player: [Player; 2],
    pub ball: Ball,
}

impl Players {
    pub fn init_players(window_size: &(u32, u32)) -> Players {
        let player_one = Player::init_player(window_size);
        let mut player_two = Player::init_player(window_size);
        let ball = Ball::new_ball(&window_size, player_one.paddle.width());

        player_two.paddle.x = window_size.0 as i32 - (10 + player_two.paddle.width() as i32);

        let player = [player_one, player_two];

        Players { player, ball }
    }
}
