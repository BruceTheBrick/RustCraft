use wgpu::RenderPass;

pub trait Renderer {
    fn render(&self, render_pass: &mut RenderPass<'_>);
}
