use wgpu::{Queue, RenderPass};

pub trait Renderer {
    fn render(&self, render_pass: &mut RenderPass<'_>, queue: &Queue);
}
