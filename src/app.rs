use crate::render_manager::RenderManager;
use crate::triangle::Triangle;
use crate::window_debug_info::WindowDebugInfo;
use crate::window_manager::WindowManager;
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
        render_manager.add_triangle(Triangle::new());
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
