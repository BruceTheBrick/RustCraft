use wgpu::{Buffer, Device, Queue, RenderPass, RenderPipeline};

use crate::{renderable::Renderer, triangle::Triangle, vertex::Vertex};

pub struct TriangleRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: Buffer,
    triangles: Vec<Triangle>,
}

impl TriangleRenderer {
    pub fn new(device: &Device) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Triangle Vertex Buffer"),
            size: (3 * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_pipeline = TriangleRenderer::create_render_pipeline(device);
        Self {
            render_pipeline,
            vertex_buffer,
            triangles: vec![],
        }
    }

    pub fn add_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles.extend(triangles);
    }

    pub fn add_triangle(&mut self, queue: &Queue, triangle: Triangle) {
        queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(triangle.vertices()),
        );
        self.triangles.push(triangle);
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
    fn render(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}
