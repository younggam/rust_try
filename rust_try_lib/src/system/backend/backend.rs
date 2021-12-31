///This is interface for application and concrete implementation
///Concrete type of this interface should implement inner logic depends on specific crate
pub trait Backend {
    ///provides executation of assembled logics
    fn run(self);

    fn set_start_task(&mut self, behavior: fn());

    fn set_reload_task(&mut self, behavior: fn());

    fn set_update_task(&mut self, behavior: fn());

    fn set_render_task(&mut self, behavior: fn());

    fn set_quit_task(&mut self, behavior: fn());
}
