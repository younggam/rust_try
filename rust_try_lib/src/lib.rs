pub use winit;

mod application;
pub use application::*;
mod structs;
pub use structs::*;

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}
