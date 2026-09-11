mod window_debug_info;
mod app;

use std::sync::Arc;

use winit::event_loop::{EventLoop, ControlFlow};
use crate::app::App;


fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
   
    
    event_loop.run_app(&mut app).unwrap();
}