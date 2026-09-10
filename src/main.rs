mod window_debug_info;

use std::sync::Arc;
use image::GenericImageView;
use window_debug_info::WindowDebugInfo;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Icon, Window, WindowId},
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
            let window_icon = create_icon();
            let window_attributes = Window::default_attributes()
                .with_title("Winit Tutorial")
                .with_window_icon(Some(window_icon))
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

fn create_icon() -> Icon {
    let png_bytes = include_bytes!("assets/logo.png");

    let image = image::load_from_memory(png_bytes)
        .expect("failed to decode window icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw_bgra();

    // Ensure the RGBA data length is valid
    assert!(rgba.len() % 4 == 0);
    assert!(width * height == (rgba.len() / 4) as u32);

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}

fn main() {
    let event_loop = EventLoop::new().unwrap();


    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
