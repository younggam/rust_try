///Sticks to raw_window_handle crate.
///Any crates that backend of window should depend on raw_window_handle.
pub trait Window: raw_window_handle::HasRawWindowHandle {
    ///Factor of scale between logical pixels and physical pixels
    ///And this might have a effect of zoom in/out.
    ///Low value : zoom out
    ///High value : zoom in
    fn scale_factor(&self) -> f64;

    //TODO: make wrapper type
    ///Phyiscal size of the window, excluding the title bar and borders.
    fn inner_size(&self) -> (u32, u32);

    fn set_inner_size(&self, width: u32, height: u32);

    //TODO: make wrapper type
    ///Phyiscal size of the window, including the title bar and borders.
    fn outer_size(&self) -> (u32, u32);

    fn set_title(&self, title: &str);

    fn set_resizable(&self, resizeable: bool);
}
