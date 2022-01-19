//! Kinda containers of extensions

use crate::*;
use application::Application;

use super::new::*;

use std::ops::Deref;
use std::os::raw::c_char;

use ash::extensions::{ext, khr};
use ash::vk;

pub trait ExtLoader: Deref {
    fn extension_names() -> Vec<*const c_char>;
}

pub struct DebugUtilsLoader {
    debug_utils: Option<ext::DebugUtils>,
    messenger_create_info: vk::DebugUtilsMessengerCreateInfoEXT,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl DebugUtilsLoader {
    pub fn new() -> Self {
        Self {
            debug_utils: None,
            messenger_create_info: NewGraphicsCoreAsh::debug_utils_messenger_create_info(),
            messenger: None,
        }
    }

    pub fn init(&mut self, debug_utils: ext::DebugUtils) {
        if self.debug_utils.is_none() {
            self.debug_utils = Some(debug_utils);
        }
    }

    pub fn create_messenger(&mut self) {
        if self.messenger.is_none() {
            self.messenger = Some(unsafe {
                self.debug_utils
                    .as_ref()
                    .expect("Use before init")
                    .create_debug_utils_messenger(&self.messenger_create_info, None)
                    .unwrap()
            });
        }
    }

    pub fn messenger_create_info(&mut self) -> &mut vk::DebugUtilsMessengerCreateInfoEXT {
        &mut self.messenger_create_info
    }

    pub fn destroy_messenger(&mut self) {
        unsafe {
            if self.debug_utils.is_some() && self.messenger.is_some() {
                self.debug_utils
                    .as_ref()
                    .unwrap()
                    .destroy_debug_utils_messenger(self.messenger.unwrap(), None);
            }
        }
    }
}

impl ExtLoader for DebugUtilsLoader {
    fn extension_names() -> Vec<*const c_char> {
        vec![ext::DebugUtils::name().as_ptr()]
    }
}

impl Deref for DebugUtilsLoader {
    type Target = ext::DebugUtils;

    fn deref(&self) -> &Self::Target {
        self.debug_utils.as_ref().expect("Use before init")
    }
}

//

//attachs winit compile error
#[cfg(target_arch = "wasm32")]
compile_error!("The platform you're compiling for is not supported by ash");
//일단, 현재 os window backend 쿼리, 관련 객체 저장
pub struct SurfaceLoader {
    surface_extension: Option<khr::Surface>,
    surface_handle: Option<vk::SurfaceKHR>,
}

impl SurfaceLoader {
    pub fn new() -> Self {
        Self {
            surface_extension: None,
            //isn't name confused?
            surface_handle: None,
        }
    }

    pub fn init(&mut self, surface_extension: khr::Surface) {
        if self.surface_extension.is_none() {
            self.surface_extension = Some(surface_extension);
        }
    }

    pub fn set_surface(&mut self, surface: vk::SurfaceKHR) {
        if self.surface_handle.is_none() {
            self.surface_handle = Some(surface);
        }
    }

    pub fn surface(&self) -> vk::SurfaceKHR {
        self.surface_handle.expect("use before set")
    }

    pub fn destroy_surface(&mut self) {
        unsafe {
            if self.surface_extension.is_some() && self.surface_handle.is_some() {
                self.surface_extension
                    .as_ref()
                    .unwrap()
                    .destroy_surface(self.surface_handle.unwrap(), None);
            }
        }
    }
}

impl ExtLoader for SurfaceLoader {
    fn extension_names() -> Vec<*const c_char> {
        if globals::APPLICATION.inited() {
            return ash_window::enumerate_required_extensions(globals::APPLICATION.window())
                .unwrap()
                .iter()
                .map(|name| name.as_ptr())
                .collect::<Vec<_>>();
        }
        #[cfg(target_os = "windows")]
        let backend_name = khr::Win32Surface::name();
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        let backend_name = {
            if let Ok(env_var) = std::env::var("WINIT_UNIX_BACKEND") {
                match env_var.as_str() {
                    "x11" => khr::XlibSurface::name(),
                    "wayland" => khr::WaylandSurface::name(),
                    _ => panic!("invalid env"),
                }
            } else {
                khr::WaylandSurface::name()
            }
        };
        #[cfg(target_os = "macos")]
        let backend_name = khr::XcbSurface::name();
        #[cfg(target_os = "android")]
        let backend_name = khr::AndroidSurface::name();
        #[cfg(target_os = "ios")]
        let backend_name = ext::MetalSurface::name();

        vec![khr::Surface::name().as_ptr(), backend_name.as_ptr()]
    }
}

impl Deref for SurfaceLoader {
    type Target = khr::Surface;

    fn deref(&self) -> &Self::Target {
        self.surface_extension.as_ref().expect("Use before init")
    }
}
