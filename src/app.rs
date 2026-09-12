use crate::renderer::Renderer;
use crate::window_debug_info::WindowDebugInfo;
use crate::window_manager::WindowManager;
use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Icon, Window, WindowId}};

#[derive(Default)]
pub struct App {
    window_manager: WindowManager,
    renderer: Option<Renderer>,
    frame_counter: WindowDebugInfo,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_manager.is_window_initialised() {
            return;
        }

        self.window_manager.init(event_loop);
        let window = Arc::clone(self.window_manager.window.as_ref().expect("window_manager.window should be initialised"));

        let renderer = pollster::block_on(Renderer::new(window)).expect("Failed to initilaise Renderer");
        self.renderer = Some(renderer);

        self.window_manager.window.as_ref().expect("window_manager.window should be initialised").request_redraw();
    }

    // This handles events sent to specific windows (e.g., resizing, keypresses, closing).
    fn window_event(&mut self, event_loop: &ActiveEventLoop,_: WindowId,event: WindowEvent,) {
        match event {
            // Triggered when the user clicks the 'X' button
            WindowEvent::CloseRequested => {
                println!("The close button was clicked; exiting...");
                event_loop.exit();
            }
            // Triggered when the window needs to redraw its contents
            WindowEvent::RedrawRequested => {
                self.frame_counter.update_fps();
                if let Some(renderer) = &mut self.renderer{
                    renderer.render(self.frame_counter.current_fps);
                }

                self.window_manager.window
                .as_ref()
                .expect("window_manager.window should be initialised")
                .request_redraw();
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer{
                    renderer.resize(size);
                }
            }
            _ => ()
        }
    }
}