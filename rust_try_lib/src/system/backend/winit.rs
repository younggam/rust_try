use crate::utils::Once;

use super::Backend;

use winit::*;

pub struct WinitBackend {
    event_loop: Once<event_loop::EventLoop<()>>,
    start_task: Option<fn()>,
    reload_task: Option<fn()>,
    update_task: Option<fn()>,
    render_task: Option<fn()>,
    quit_task: Option<fn()>,
}

impl WinitBackend {
    pub fn new() -> Self {
        Self {
            event_loop: Once::new(event_loop::EventLoop::new()),
            start_task: None,
            reload_task: None,
            update_task: None,
            render_task: None,
            quit_task: None,
        }
    }
}

impl Backend for WinitBackend {
    fn run(mut self) {
        if let Some(ref f) = self.start_task {
            f();
        }
        self.event_loop
            .consume()
            .run(move |event, _, _control_flow| {
                if let Some(ref f) = self.update_task {
                    f();
                }
                match event {
                    event::Event::NewEvents(_) => {}
                    event::Event::WindowEvent { .. } => {}
                    event::Event::DeviceEvent { .. } => {}
                    event::Event::UserEvent(_) => {}
                    event::Event::Suspended => {}
                    event::Event::Resumed => {}
                    event::Event::MainEventsCleared => {
                        if let Some(ref f) = self.render_task {
                            f();
                        }
                    }
                    event::Event::RedrawRequested(_) => {}
                    event::Event::RedrawEventsCleared => {}
                    event::Event::LoopDestroyed => {
                        if let Some(ref f) = self.quit_task {
                            f();
                        }
                    }
                }
            });
    }

    fn set_start_task(&mut self, task: fn()) {
        self.start_task = Some(task);
    }

    fn set_reload_task(&mut self, task: fn()) {
        self.reload_task = Some(task);
    }

    fn set_update_task(&mut self, task: fn()) {
        self.update_task = Some(task);
    }

    fn set_render_task(&mut self, task: fn()) {
        self.render_task = Some(task);
    }

    fn set_quit_task(&mut self, task: fn()) {
        self.quit_task = Some(task);
    }
}
