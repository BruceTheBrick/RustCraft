mod window_debug_info;

use std::sync::Arc;
use window_debug_info::WindowDebugInfo;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{Window, WindowId},
};


#[derive(Default)]
struct App {
    // We wrap the window in an Option because it can only be initialized 
    // once the event loop resumes (especially important for mobile/web platforms).
    window: Option<Arc<Window>>,
    frame_counter: WindowDebugInfo,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Winit Tutorial")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window);
        }
    }

    // This handles events sent to specific windows (e.g., resizing, keypresses, closing).
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // Ensure the event belongs to our window instance
        if let Some(window) = &self.window {
            if window.id() != window_id {
                return;
            }
        }

        self.frame_counter.update_fps();
        match event {
            // Triggered when the user clicks the 'X' button
            WindowEvent::CloseRequested => {
                println!("The close button was clicked; exiting...");
                event_loop.exit();
            }
            // Triggered when the window needs to redraw its contents
            WindowEvent::RedrawRequested => {
                // This is where you would place your rendering code (wgpu, vulkano, etc.)
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => ()
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();


    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
