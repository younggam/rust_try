use super::core::*;

pub struct InnerRenderer<T: Core> {
    core: T,
}

#[cfg(feature = "vulkan")]
impl InnerRenderer<CoreVulkan> {
    pub fn new() -> Self {
        Self {
            core: CoreVulkan::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.core.initialize();
    }

    pub fn render(&mut self) {
        self.core.render();
    }
}

#[cfg(feature = "vulkan")]
pub type Renderer = InnerRenderer<CoreVulkan>;
