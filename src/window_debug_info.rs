use std::time::Instant;

pub struct WindowDebugInfo{
    last_frame: Instant,
    frame_count: u32,
    fps_timer: Instant,
    current_fps: f32,
}

impl Default for WindowDebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowDebugInfo{
    pub fn new () -> Self{
        Self{
            last_frame: Instant::now(),
            frame_count: 0,
            fps_timer: Instant::now(),
            current_fps: 0.0,
        }
    }

    pub fn update_fps(&mut self){
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.frame_count += 1;

        if self.fps_timer.elapsed().as_secs_f32() >= 1.0 {
            self.current_fps = self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
            self.frame_count = 0;
            self.fps_timer = now;
        }

        print!("\rFPS: {:.2}", self.current_fps);
    }
}