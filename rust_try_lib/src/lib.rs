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

#[macro_use]
extern crate lazy_static;

pub mod application {
    leaf_mod! {pub application}
    leaf_mod! {pub module}
    #[cfg(feature = "winit")]
    leaf_mod! {pub winit}
}

pub mod globals {
    leaf_mod! {pub event}
    leaf_mod! {pub globals}
    leaf_mod! {pub input}
    leaf_mod! {pub time}
}

pub mod graphics {
    #[cfg(feature = "vulkan")]
    pub mod ash {
        leaf_mod! {pub core}
        leaf_mod! {pub extensions}
        leaf_mod! {pub new}
    }

    pub mod elements {
        leaf_mod! {pub material}
        leaf_mod! {pub vertex}
    }

    pub mod window {
        leaf_mod! {pub window}
        #[cfg(feature = "winit")]
        leaf_mod! {pub winit}
    }

    leaf_mod! {pub core}
}

pub mod math {
    leaf_mod! {pub vector}
    leaf_mod! {pub matrix}
}

pub mod utils {
    leaf_mod! {pub macros}
    leaf_mod! {pub wrapper}
}
