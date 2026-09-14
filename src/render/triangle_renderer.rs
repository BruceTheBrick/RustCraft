use wgpu::{Buffer, Device, Queue, RenderPass};

use crate::{renderable::Renderer, triangle::Triangle, vertex::Vertex};

pub struct TriangleRenderer {
    render_pipeline: wgpu::RenderPipeline,
    staging_buffer: Buffer,
    vertex_buffer: Buffer,
    triangles: Vec<Triangle>,
}
const TRIANGLES_PER_BATCH: usize = 10;
const VERTICES_PER_TRIANGLE: usize = 3;
const BUFFER_CAPACITY_IN_TRIANGLES: usize = 150;

impl TriangleRenderer {
    pub fn new(device: &Device) -> Self {
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Triangle Vertex Staging Buffer"),
            size: (BUFFER_CAPACITY_IN_TRIANGLES
                * VERTICES_PER_TRIANGLE
                * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Triangle Vertex Buffer"),
            size: (TRIANGLES_PER_BATCH
                * VERTICES_PER_TRIANGLE
                * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_pipeline = TriangleRenderer::create_render_pipeline(device);
        Self {
            render_pipeline,
            staging_buffer,
            vertex_buffer,
            triangles: vec![],
        }
    }

    pub fn add_triangles(&mut self, triangles: Vec<Triangle>) {
        for triangle in triangles {
            self.add_triangle(triangle);
        }
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn prepare(&self, queue: &Queue){
        let mut vertices = Vec::new();
        for triangle in &self.triangles{
            vertices.extend_from_slice(triangle.vertices());
        }

        queue.write_buffer(
                &self.staging_buffer,
                0,
                bytemuck::cast_slice(&vertices),
            );
    }

    pub fn copy_batch(
    &self,
    command_encoder: &mut wgpu::CommandEncoder,
    staging_offset: u64,
    triangle_count: usize,
) {
    let vertex_count = triangle_count * VERTICES_PER_TRIANGLE;

    let size = (vertex_count * std::mem::size_of::<Vertex>()) as u64;

    command_encoder.copy_buffer_to_buffer(
        &self.staging_buffer,
        staging_offset,
        &self.vertex_buffer,
        0,
        size,
    );
}

    fn create_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(crate::vertex::SHADER.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Some(Vertex::get_layout_buffer())],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        return render_pipeline;
    }
}

impl Renderer for TriangleRenderer {
    fn render(&self, render_pass: &mut RenderPass<'_>, queue: &Queue) {
        println!("Rendering {} triangles", self.triangles.len());
        let mut batch_count = 0;

        for x in 0..self.triangles.len() {
            batch_count += 1;

            if batch_count == TRIANGLES_PER_BATCH {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

                render_pass.draw(0..(batch_count * VERTICES_PER_TRIANGLE) as u32, 0..1);

                batch_count = 0;
            }
        }

        if batch_count > 0 {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.draw(0..(batch_count * VERTICES_PER_TRIANGLE) as u32, 0..1);
        }
    }
}
