pub trait GraphicsCore {
    fn init(&mut self, window: &dyn raw_window_handle::HasRawWindowHandle);

    fn render(&mut self);
}
