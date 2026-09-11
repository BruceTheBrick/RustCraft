use std::sync::Arc;
use wgpu::{SurfaceColorSpace::Auto};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontRef};
use wgpu_text::glyph_brush::{Section as TextSection, Text};
use winit::window;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Icon, Window, WindowId}};
use crate::window_debug_info::WindowDebugInfo;
use pollster;

#[derive(Default)]
pub struct App {
    // We wrap the window in an Option because it can only be initialized 
    // once the event loop resumes (especially important for mobile/web platforms).
    pub window: Option<Arc<Window>>,
    frame_counter: WindowDebugInfo,
    surface: Option<wgpu::Surface<'static>>,
    gpu_instance: wgpu::Instance,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    textbrush: Option<TextBrush<FontRef<'static>>>,
}

impl App {
    fn configure_surface(&mut self){
        let window = self.window.as_ref().expect("Window is not initialized");
        let surface = self.surface.as_mut().expect("Surface is not initialized");
        let adapter = self.adapter.as_ref().expect("Adapter is not initialized");
        let device = self.device.as_ref().expect("Device is not initialized");

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0]; // Choose the first supported format
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: Auto,
        };

        surface.configure(&device, &config);
        self.build_textbrush(surface_format);
    }
    
    fn handle_redraw(&mut self) {
        let surface = self.surface.as_ref().expect("Surface is not initialized");
        let device = self.device.as_ref().expect("Device is not initialized");

        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded => return,
            wgpu::CurrentSurfaceTexture::Outdated => {
                eprintln!("Surface needs to be reconfigured");
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                eprintln!("Surface was lost and must be recreated");
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("Surface texture validation failed");
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let fps_text = format!("FPS: {:.2}", self.get_framerate());
        let text_brush = self.textbrush.as_mut().expect("TextBrush is not initialized");
        let section = TextSection::default()
            .with_screen_position((10.0, 10.0))
            .with_text(vec![Text::new(&fps_text)
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(24.0)]);

        let queue = self.queue.as_ref().expect("Queue is not initialized");
        let _ = text_brush.queue(device, queue, vec![section]);
        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.08,
                            b: 50.12,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            text_brush.draw(&mut render_pass);
        }

        let queue = self.queue.as_ref().expect("Queue is not initialized");
        queue.submit(std::iter::once(encoder.finish()));
        queue.present(output);
    }

    fn init_default_window(&mut self, event_loop: &ActiveEventLoop){
        let window_attributes = Window::default_attributes()
                .with_title("Winit Tutorial")
                .with_window_icon(App::create_icon())
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
            
        self.window = Some(Arc::new(event_loop.create_window(window_attributes).unwrap()));
    }

    fn create_icon() -> Option<Icon> {
        let png_bytes = include_bytes!("assets/logo.png");

        let image = image::load_from_memory(png_bytes)
            .expect("failed to decode window icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw_bgra();

        // Ensure the RGBA data length is valid
        assert!(rgba.len() % 4 == 0);
        assert!(width * height == (rgba.len() / 4) as u32);

        return Some(Icon::from_rgba(rgba, width, height).expect("Failed to create icon"));
    }
    
    fn build_textbrush(&mut self, surface_format: wgpu::TextureFormat) {
        let device = self.device.as_ref().expect("Device is not initialized");
        let font = include_bytes!("assets/font.ttf");
        let brush = BrushBuilder::using_font_bytes(font)
        .expect("Failed to load font")
        .build(device, 800, 600, surface_format);
        self.textbrush = Some(brush);
    }

    fn get_framerate(&self) -> f32 {
        self.frame_counter.current_fps
    }
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {

            self.init_default_window(event_loop);
            let window = self.window.as_mut().expect("Window is not initialized");

            self.gpu_instance = wgpu::Instance::default();

            let surface = self.gpu_instance.create_surface(Arc::clone(&window)).expect("Failed to create surface");

            self.surface = Some(surface);

            self.adapter = Some(pollster::block_on(self.gpu_instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: self.surface.as_ref(),
                    force_fallback_adapter: false,
                    apply_limit_buckets: true,
                },
            )).expect("Failed to find an appropriate adapter"));

            let adapter = self.adapter.as_ref().expect("Adapter is not initialized");
            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    trace: wgpu::Trace::Off,
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
            )).expect("Failed to create device");

            self.device = Some(device);
            self.queue = Some(queue);
            self.configure_surface();

            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    // This handles events sent to specific windows (e.g., resizing, keypresses, closing).
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // Ensure the event belongs to our window instance
        if let Some(window) = &self.window {
            if window.id() != window_id {
                return;
            }
        }

        self.frame_counter.update_fps();

        match event {
            // Triggered when the user clicks the 'X' button
            WindowEvent::CloseRequested => {
                println!("The close button was clicked; exiting...");
                event_loop.exit();
            }
            // Triggered when the window needs to redraw its contents
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            _ => ()
        }
    }
}