use crate::system::Main;

pub struct Application {
    main: Main,
}

impl Application {
    pub fn new() -> Self {
        Self { main: Main::new() }
    }

    pub fn run(self) {
        self.main.run();
    }
}
