use crate::context::Context;

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::ActiveEventLoop,
    window::Window,
};

pub struct Application {
    context: Option<Context>,
}

impl Application {
    pub fn new() -> Self {
        Application { context: None }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let None = self.context {
            let window_attributes = Window::default_attributes().with_title("Mingine");
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
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        };
    }
}
