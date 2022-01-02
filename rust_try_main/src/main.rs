use rust_try_lib::application::*;

use modules::core::CoreModule;

fn main() {
    let app = ApplicationWinit::new(CoreModule {});
    app.run();
}

mod modules {
    pub mod core;
    // pub mod renderer;
}
