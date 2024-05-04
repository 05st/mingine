use crate::application::Application;

use winit::event_loop::EventLoop;

mod application;
mod context;

pub fn run() {
    env_logger::init();

    let mut app = Application::new();

    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut app)
        .expect("error when running app");
}
