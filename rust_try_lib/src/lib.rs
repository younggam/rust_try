mod application;
pub use application::*;

/*pub mod graphics {
    pub mod elements {
        mod material;
        pub use material::*;

        mod vertex;
        pub use vertex::*;
    }

    mod renderer;
    pub use renderer::*;
}

pub mod math {
    mod vector;
    pub use vector::*;

    mod matrix;
    pub use matrix::*;
}*/

pub mod utils {
    mod macros;
    pub(crate) use macros::*;

    mod wrapper;
    pub use wrapper::*;
}
