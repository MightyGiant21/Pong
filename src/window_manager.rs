use sdl2::{render::Canvas, Sdl, video::Window, VideoSubsystem, pixels::Color};



pub struct WindowManager {
    pub canvas: Canvas<Window>,
    bg_col: Color
}

impl WindowManager {
    pub fn build_window(sdl_context: &Sdl) -> WindowManager {
        let v_subsys = create_video_sub_sys(&sdl_context); 
        let window = create_window(v_subsys);
        let canvas = create_canvas(window);        
        let bg_col = Color::RGB(0, 0, 0);

        WindowManager { canvas, bg_col }
    }

    pub fn refresh(&mut self) {
        self.canvas.set_draw_color(self.bg_col);
        self.canvas.present();
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