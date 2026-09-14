use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    renderable::Renderer, renderer::BaseRenderer, triangle::Triangle,
    triangle_renderer::TriangleRenderer,
};

pub struct RenderManager {
    base_renderer: BaseRenderer,
    triangle_renderer: TriangleRenderer,
}

impl RenderManager {
    pub async fn new(window: Arc<Window>) -> Self {
        let base_renderer = BaseRenderer::new(window)
            .await
            .expect("base_renderer could not be created");
        let triangle_renderer = TriangleRenderer::new(&base_renderer.device());
        return Self {
            base_renderer,
            triangle_renderer,
        };
    }

    pub fn render(&mut self) {
        let triangle_renderer = &self.triangle_renderer;
        self.base_renderer.render(|render_pass, queue| {
            triangle_renderer.render(render_pass, queue);
        });
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {}

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangle_renderer.add_triangle(triangle);
    }

    pub fn add_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangle_renderer.add_triangles(triangles);
    }
}
