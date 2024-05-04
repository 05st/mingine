use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{self, ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

struct WindowState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>
}

struct Application<'a> {
    window_state: Option<WindowState<'a>>,
}

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let None = self.window_state {
            let window_attributes = Window::default_attributes().with_title("Mingine");
            let window_obj = event_loop.create_window(window_attributes).expect("failed to create window");
            let window = Arc::new(window_obj);

            let size = window.inner_size();

            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
            
            let surface = instance.create_surface(window).expect("failed to create surface");

            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            })).unwrap();

            let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

            let surface_caps = surface.get_capabilities(&adapter);
            let surface_fmt = surface_caps.formats.iter()
                .copied()
                .filter(|f| f.is_srgb())
                .next()
                .unwrap_or(surface_caps.formats[0]);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_fmt,
                width: size.width,
                height: size.height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2
            };

            self.window_state = Some(WindowState {
                surface: surface,
                device: device,
                queue: queue,
                config: config,
                size: size,
                window: Arc::clone(&window)
            });
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                window.request_redraw();
            },
            _ => {}
        };
    }
}

pub fn run() {
    env_logger::init();

    let mut app = Application {
        window: None
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
