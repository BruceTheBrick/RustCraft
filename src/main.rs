mod window_debug_info;
mod app;
#[path="models/vertex.rs"]
mod vertex;
#[path="models/triangle.rs"]
mod triangle;

#[path="render/base_renderer.rs"]
mod renderer;
#[path ="render/render_pipeline.rs"]
mod render;
#[path="render/window_manager.rs"]
mod window_manager;
#[path="render/renderable.rs"]
mod renderable;
#[path="render/render_manager.rs"]
mod render_manager;
#[path="render/triangle_renderer.rs"]
mod triangle_renderer;

use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default());
}