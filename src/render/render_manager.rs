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

        self.base_renderer.render(|command_encoder, texture_view, queue| {
        {
            triangle_renderer.prepare(queue);

            triangle_renderer.copy_batch(command_encoder,0,TRIANGLES_PER_BATCH);

            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view, // write the results into our surface texture
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Clear the whole texture to this colour at the start
                        // of the pass (RGBA on a 0–1 scale)…
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store, // …and keep the result in the texture
                    },
                    depth_slice: None, // 2D target, so no depth slice
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            
            triangle_renderer.render(&mut render_pass, queue);
        }
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
