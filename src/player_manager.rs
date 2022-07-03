use sdl2::rect::Rect;

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
    pub top_coord: i32,
    pub bot_coord: i32,
}

impl Player {
    pub fn init_player(window_size: &(u32, u32)) -> Player {
        let x = 10;
        let y = (window_size.1 / 2) as i32;
        let width = window_size.0 / 70;
        let height = window_size.1 / 10;
        let paddle = Rect::new(x, y, width, height);
        let top_coord = (window_size.1 as i32 / 2) + (height / 2) as i32;
        let bot_coord = top_coord + height as i32;

        Player {
            paddle,
            top_coord,
            bot_coord,
        }
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
