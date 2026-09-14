use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

pub const SHADER: &str = r#"
    struct VertexInput{
        @location(0) position: vec2<f32>,
        @location(1) color: vec3<f32>,
    };

    struct VertexOutput{
        @builtin(position) position: vec4<f32>,
        @location(0) color: vec3<f32>,
    };

    @vertex
    fn vs_main(in: VertexInput) -> VertexOutput{
        var out: VertexOutput;
        out.color = in.color;
        out.position = vec4<f32>(in.position, 0.0, 1.0);
        return out;
    }

    @fragment
    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
        return vec4<f32>(in.color, 1.0);
    }
    "#;

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }

    pub fn get_layout_buffer() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
