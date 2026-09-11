use std::time::Instant;

pub struct WindowDebugInfo{
    pub last_frame: Instant,
    pub frame_count: u32,
    pub fps_timer: Instant,
    pub current_fps: f32,
}

impl Default for WindowDebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowDebugInfo{
    const FPS_UPDATE_SECONDS: f32 = 1.0;

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
        self.last_frame = now;
        self.frame_count += 1;

        // Update the FPS every second
        if self.fps_timer.elapsed().as_secs_f32() >= Self::FPS_UPDATE_SECONDS{
            self.current_fps = self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
            self.frame_count = 0;
            self.fps_timer = now;
        }

        print!("\rFPS: {:.2}", self.current_fps);
    }
}