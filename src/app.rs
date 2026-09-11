use std::sync::Arc;

use wgpu::{DeviceDescriptor, SurfaceColorSpace::Auto, wgc::device};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
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
    }
    
    fn handle_redraw(&self) {
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
        
        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.08,
                            b: 0.12,
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
        }

        let queue = self.queue.as_ref().expect("Queue is not initialized");
        queue.submit(std::iter::once(encoder.finish()));
        queue.present(output);
    }

    fn init_default_window(&mut self, event_loop: &ActiveEventLoop){
        let window_attributes = Window::default_attributes()
                .with_title("Winit Tutorial")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
            
        self.window = Some(Arc::new(event_loop.create_window(window_attributes).unwrap()));
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
                    apply_limit_buckets: todo!(),
                },
            )).expect("Failed to find an appropriate adapter"));

            let (device, queue) = pollster::block_on(self.adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    trace: wgpu::Trace::Off,
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
            )).expect("Failed to create device");

            self.device = device;
            self.queue = queue;
            self.configure_surface();
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