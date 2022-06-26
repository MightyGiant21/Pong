use sdl2::{render::Canvas, Sdl, video::Window, VideoSubsystem, pixels::Color, EventPump, rect::Rect};

use crate::player_manager::{Players, Player};



pub struct WindowManager {
    pub canvas: Canvas<Window>,
    bg_col: Color,
    paddle_col: Color,
    pub window_size: (u32, u32),
    pub event_pump: EventPump
}

impl WindowManager {
    pub fn build_window() -> WindowManager {
        let sdl_context = sdl2::init().unwrap();
        let v_subsys = create_video_sub_sys(&sdl_context); 
        let window = create_window(v_subsys);
        let window_size = window.size();
        let canvas = create_canvas(window);        
        let bg_col = Color::RGB(0, 0, 0);
        let paddle_col = Color::RGB(0, 255, 0);
        let event_pump = sdl_context.event_pump().unwrap();

        WindowManager { canvas, bg_col, paddle_col, window_size , event_pump}
    }

    pub fn refresh(&mut self, players: &mut Players) {
        self.canvas.set_draw_color(self.bg_col);
        self.canvas.present();
        self.render_paddles(players);
        self.render_ball(&players.player[0]);
        self.canvas.present();
    }
    
    fn render_paddles(&mut self, players: &mut Players) {
        self.canvas.set_draw_color(self.paddle_col);
        self.canvas.fill_rect(players.player[0].paddle).unwrap();
        self.canvas.fill_rect(players.player[1].paddle).unwrap();
    }

    fn render_ball(&mut self, player: &Player) {
        let size = player.paddle.width();
        let ball = Rect::new(self.window_size.0 as i32 / 2, self.window_size.1 as i32 / 2, size, size);
        self.canvas.fill_rect(ball).unwrap();
    }
}

fn create_canvas(window: Window) -> Canvas<Window> {
    let canvas = window.into_canvas().build();

    match canvas {
        Ok(can) => return can,
        Err(e) => panic!("Failed to build canvas: {}", e)
    }
}

fn create_window(v_subsys: VideoSubsystem) -> Window {
    let window = v_subsys
    .window("Pong", 800, 800)
    .fullscreen_desktop()
    .build();

    match window {
        Ok(win) => return win,
        Err(e) => panic!("Failed to build window: {}", e)
    }
}

fn create_video_sub_sys(sdl_context: &Sdl) -> VideoSubsystem {
    let v_subsys = sdl_context.video(); 

    match v_subsys {
        Ok(v_sub) => {return v_sub},
        Err(e) => {panic!("Failed to create a video subsystem: {}", e)}
    }
}