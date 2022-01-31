/*!
## Directory Structure
Each directory(folder) shouldn't have it's own contents.

All the contents should from leafs(file).

But in use, contents can be accessed by real directory paths.
*/

//To keep the DRY when adding leaf modules
macro_rules! leaf_mod{
    {$visibility:vis $identifier:ident}=>{
        mod $identifier;
        $visibility use self::$identifier::*;
    }
}

// #[macro_use]
// extern crate lazy_static;

pub mod application {
    leaf_mod! {pub application}
    leaf_mod! {pub event}
    leaf_mod! {pub scene}
    #[cfg(feature = "winit")]
    leaf_mod! {pub winit}
}

pub mod graphics {
    pub mod elements {
        leaf_mod! {pub material}
        leaf_mod! {pub vertex}
    }

    pub mod wgpu {
        leaf_mod! {pub core}
    }

    pub mod window {
        leaf_mod! {pub window}
        #[cfg(feature = "winit")]
        leaf_mod! {pub winit}
    }

    leaf_mod! {pub core}
}

pub mod input {
    pub mod keyboard;
}

pub mod math {
    leaf_mod! {pub vector}
    leaf_mod! {pub matrix}
}

pub mod utils {
    leaf_mod! {pub macros}
    leaf_mod! {pub wrapper}
}

pub mod time;
