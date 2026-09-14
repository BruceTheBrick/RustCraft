use crate::triangle::Triangle;
use crate::window_debug_info::WindowDebugInfo;
use crate::window_manager::WindowManager;
use crate::{render_manager::RenderManager, vertex::Vertex};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

#[derive(Default)]
pub struct App {
    window_manager: WindowManager,
    render_manager: Option<RenderManager>,
    frame_counter: WindowDebugInfo,
}

impl App {
    fn get_triangles() -> Vec<Triangle> {
        const GRID_SIZE: usize = 11;
        const STEP: f32 = 0.18;
        const HALF: f32 = 0.06;

        let mut triangles = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = -0.9 + (col as f32) * STEP;
                let y = 0.9 - (row as f32) * STEP;

                let r = (col as f32 / GRID_SIZE as f32).clamp(0.0, 1.0);
                let g = (row as f32 / GRID_SIZE as f32).clamp(0.0, 1.0);
                let b = 0.5 + 0.5 * ((col + row) as f32 / (GRID_SIZE * 2) as f32);
                let color = [r, g, b];

                // Counter-clockwise order for an upward-pointing triangle.
                let left = [x - HALF, y - HALF];
                let right = [x + HALF, y - HALF];
                let top = [x, y + HALF];

                triangles.push(Triangle::from_vertices([
                    Vertex::new(left, color),
                    Vertex::new(right, color),
                    Vertex::new(top, color),
                ]));
            }
        }

        triangles
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_manager.is_window_initialised() {
            return;
        }

        self.window_manager.init(event_loop);
        let window = Arc::clone(
            self.window_manager
                .window
                .as_ref()
                .expect("window_manager.window should be initialised"),
        );
        self.window_manager
            .window
            .as_ref()
            .expect("window_manager.window should be initialised")
            .request_redraw();

        let mut render_manager = pollster::block_on(RenderManager::new(window));
        render_manager.add_triangles(App::get_triangles());
        self.render_manager = Some(render_manager);
    }

    // This handles events sent to specific windows (e.g., resizing, keypresses, closing).
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            // Triggered when the user clicks the 'X' button
            WindowEvent::CloseRequested => {
                println!("The close button was clicked; exiting...");
                event_loop.exit();
            }
            // Triggered when the window needs to redraw its contents
            WindowEvent::RedrawRequested => {
                self.frame_counter.update_fps();

                if let Some(render_manager) = &mut self.render_manager {
                    render_manager.render();
                }

                self.window_manager
                    .window
                    .as_ref()
                    .expect("window_manager.window should be initialised")
                    .request_redraw();
            }

            WindowEvent::Resized(size) => {
                if let Some(render_manager) = &mut self.render_manager {
                    render_manager.resize(size);
                }
            }
            _ => (),
        }
    }
}
