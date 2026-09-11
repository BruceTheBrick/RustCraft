mod window_debug_info;
mod app;

use std::sync::Arc;

use winit::event_loop::{EventLoop, ControlFlow};
use crate::app::App;


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