use std::sync::Arc;

use winit::window::Window;

use crate::{renderer::BaseRenderer, triangle::Triangle, triangle_renderer::TriangleRenderer};

pub struct RenderManager{
    base_renderer: BaseRenderer,
    triangle_renderer: TriangleRenderer,
}

impl RenderManager{
    pub async fn new(window: Arc<Window>) -> Self{
        let base_renderer = BaseRenderer::new(window).await.expect("base_renderer could not be created");
        let triangle_renderer = TriangleRenderer::new(&base_renderer.device);
        return Self { base_renderer, triangle_renderer};
    }
    
    pub fn add_triangle(&mut self, triangle: Triangle){
        self.triangle_renderer.add_triangle(triangle);
    }
}