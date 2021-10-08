pub use winit;

mod application;
pub use application::*;

pub mod graphics {
    pub mod elements {
        mod material;
        pub use material::*;
        
        mod vertex;
        pub use vertex::*;
    }
}

pub mod utils {
    mod macros;
    pub(crate) use macros::*;
}
