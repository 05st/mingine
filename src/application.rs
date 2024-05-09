use crate::context::Context;

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct Application {
    pub context: Option<Context>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let None = self.context {
            let window_attributes = Window::default_attributes()
                .with_title("Mingine");
            let window = event_loop
                .create_window(window_attributes)
                .expect("failed to create window");
            let context = pollster::block_on(Context::new(Arc::new(window)));
            self.context = Some(context);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let ctx = self.context.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                ctx.resize(physical_size);
                ctx.window.request_redraw();
            },
            WindowEvent::RedrawRequested if window_id == ctx.window.id() => {
                ctx.window.request_redraw(); // request the next frame (draws in vsync)

                match ctx.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => ctx.resize(ctx.size),
                    Err(e) => eprintln!("{:?}", e),
                };
            },
            WindowEvent::CursorMoved { device_id: _, position } => {
                ctx.clear_color = wgpu::Color {
                    r: (position.x / 200.0).sin(),
                    g: (position.y / 200.0).cos(),
                    b: 0.3,
                    a: 1.0,
                };
            },
            _ => (),
        };
    }
}
