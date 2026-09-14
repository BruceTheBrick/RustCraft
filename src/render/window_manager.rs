use std::sync::Arc;

use winit::{
    event_loop::ActiveEventLoop,
    window::{Icon, Window, WindowId},
};

#[derive(Default)]
pub struct WindowManager {
    pub window: Option<Arc<Window>>,
    pub window_id: Option<WindowId>,
}

impl WindowManager {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Winit Tutorial")
            .with_window_icon(WindowManager::create_icon())
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        let window = event_loop.create_window(attributes).unwrap();
        self.window_id = Some(window.id());
        self.window = Some(Arc::new(window));
    }

    pub fn is_window_initialised(&self) -> bool {
        return self.window.is_some();
    }

    // Private Functions
    fn create_icon() -> Option<Icon> {
        let png_bytes = include_bytes!("../assets/logo.png");

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
}
