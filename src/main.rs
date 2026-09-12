mod window_debug_info;
mod app;
mod renderer;
#[path="vertex/vertex.rs"]
mod vertex;

#[path ="render/render_pipeline.rs"]
mod render;

#[path="render/window_manager.rs"]
mod window_manager;


use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default());
}