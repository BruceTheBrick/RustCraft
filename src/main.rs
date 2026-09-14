mod app;
#[path = "models/triangle.rs"]
mod triangle;
#[path = "models/vertex.rs"]
mod vertex;
mod window_debug_info;

#[path = "render/render_manager.rs"]
mod render_manager;
#[path = "render/renderable.rs"]
mod renderable;
#[path = "render/base_renderer.rs"]
mod renderer;
#[path = "render/triangle_renderer.rs"]
mod triangle_renderer;
#[path = "render/window_manager.rs"]
mod window_manager;

use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default());
}
