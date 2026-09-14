use std::sync::Arc;
use wgpu::{Device, Queue};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontRef};
use winit::window::Window;

pub struct BaseRenderer {
    // GPU State Management
    device: wgpu::Device,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // UI Components to Render
    text_brush: TextBrush<FontRef<'static>>,
}

impl BaseRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance
            .create_surface(window.clone())
            .map_err(RendererError::SurfaceCreation)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
            .map_err(|_| RendererError::AdapterNotFound)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(RendererError::DeviceCreation)?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| surface_caps.formats.first().copied())
            .ok_or(RendererError::NoSurfaceFormat)?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        surface.configure(&device, &config);

        let text_brush = BaseRenderer::build_textbrush(&device, &config);
        Ok(Self {
            surface,
            device,
            queue,
            config,
            text_brush,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.text_brush
            .resize_view(new_size.width as f32, new_size.height as f32, &self.queue);
    }

    pub fn render<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut wgpu::RenderPass<'_>, &mut wgpu::Queue),
    {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,

            // Transient conditions (minimised, timed out) — skip this frame.
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return,

            // Surface needs reconfiguring — re-apply the current config, then skip.
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => return,
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // let fps_text = format!("FPS: {:.0}", fps);
        // let section = TextSection::default()
        //     .with_screen_position((10.0, 10.0))
        //     .with_text(vec![Text::new(&fps_text)
        //         .with_color([1.0, 1.0, 1.0, 1.0])
        //         .with_scale(24.0)]);
        // self.text_brush.queue(&self.device, &self.queue, vec![section]);

        // render_state.prepare_renderables();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, // write the results into our surface texture
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

            callback(&mut render_pass, &mut self.queue);
            // render_state.draw_renderables(&mut render_pass);
            // self.text_brush.draw(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);
    }

    fn build_textbrush(
        device: &Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> TextBrush<FontRef<'static>> {
        let font = include_bytes!("../assets/font.ttf");
        let brush = BrushBuilder::using_font_bytes(font)
            .expect("Failed to load font")
            .build(&device, config.width, config.height, config.format);
        return brush;
    }

    pub fn device(&self) -> &Device {
        return &self.device;
    }

    pub fn queue(&self) -> &Queue {
        return &self.queue;
    }
}

#[derive(Debug)]
pub enum RendererError {
    SurfaceCreation(wgpu::CreateSurfaceError),
    AdapterNotFound,
    DeviceCreation(wgpu::RequestDeviceError),
    NoSurfaceFormat,
}
