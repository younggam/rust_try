pub trait RendererCore {
    fn init(&mut self, window: &dyn raw_window_handle::HasRawWindowHandle);

    fn render(&mut self);
}
