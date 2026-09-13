use wgpu::{Device, RenderPass};

use crate::{renderable::Renderer, triangle::Triangle};

pub struct TriangleRenderer{
    triangles: Vec<Triangle>
}

impl TriangleRenderer{
    pub fn new(device: &Device) -> Self{
        Self{
            triangles: vec![]
        }
    }

    pub fn add_triangles(&mut self, triangles: Vec<Triangle>){
        self.triangles.extend(triangles);
    }

    pub fn add_triangle(&mut self, triangle: Triangle){
        self.triangles.push(triangle);
    }
}

impl Renderer for TriangleRenderer{
    fn render(&self, render_pass: &mut RenderPass<'_>){
        let _ = render_pass;
        render_pass.draw
    }
}