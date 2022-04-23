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
#[macro_use]
extern crate wgpu;

pub mod application {
    leaf_mod! {pub application}
    leaf_mod! {pub event}
    leaf_mod! {pub scene}
}

pub mod graphics {
    pub mod elements {
        leaf_mod! {pub model}
        leaf_mod! {pub texture}
        leaf_mod! {pub vertex}
    }

    leaf_mod! {pub graphics}
}

pub mod inputs {
    leaf_mod! {pub buttons}
    leaf_mod! {pub cursor}
    leaf_mod! {pub inputs}
    leaf_mod! {pub keyboard}
    leaf_mod! {pub mock}
    leaf_mod! {pub mouse}
}

pub mod utils {
    leaf_mod! {pub macros}
    leaf_mod! {pub time}
    leaf_mod! {pub utils}
    leaf_mod! {pub wrapper}
}

pub use cgmath;
pub use winit;
