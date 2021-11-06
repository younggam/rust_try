/*!
##Directory Structure
Each directory(folder) shouldn't have it's own contents.
All the contents should from leafs(file).
But in use, contents can be accessed by real directory paths.
*/

///To keep the DRY when adding leaf modules
macro_rules! leaf_mod{
    {$visibility:vis $identifier:ident}=>{
        mod $identifier;
        $visibility use $identifier::*;
    }
}

leaf_mod! {pub application}

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
    leaf_mod! {pub macros}
    leaf_mod! {pub wrapper}
}
