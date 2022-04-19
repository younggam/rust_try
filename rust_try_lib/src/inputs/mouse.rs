use super::inputs::ElementState;

use cgmath::*;

use winit::event::*;

pub struct Mouse {
    motion: Vector2<f32>,
    wheel: f32,

    left: ElementState,
    middle: ElementState,
    right: ElementState,
}
