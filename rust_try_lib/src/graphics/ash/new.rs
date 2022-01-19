//!Hope this module could offer global draw functionality.

use crate::*;
use application::Application;
use utils::LazyManual;

use graphics::ash::extensions::*;

use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

use ash::extensions::{ext, khr};
use ash::vk;

pub struct NewGraphicsCoreAsh {
    entry: ash::Entry,

    instance: LazyManual<ash::Instance>,
    debug_utils_loader: DebugUtilsLoader,
    surface_loader: SurfaceLoader,

    physical_device: vk::PhysicalDevice,
}

///Constructor and initializer.
impl NewGraphicsCoreAsh {
    pub fn new() -> Self {
        Self {
            entry: ash::Entry::linked(),

            instance: LazyManual::new(),
            debug_utils_loader: DebugUtilsLoader::new(),
            surface_loader: SurfaceLoader::new(),

            physical_device: vk::PhysicalDevice::null(),
        }
    }

    pub fn init(&mut self) {
        self.init_instance();
        self.init_debug_utils();
        self.init_surface();

        self.init_physical_device();
    }
}

///getter
// impl GraphicsCoreAsh {
//     pub fn entry(&self) -> &ash::Entry {
//         &self.entry
//     }
//
//     pub fn instance(&self) -> &ash::Instance {
//         &self.instance
//     }
// }

///Constants or same as.
impl NewGraphicsCoreAsh {
    //if I know correctly, layers aren't needed for release verion.
    const ENABLE_LAYERS: bool = cfg!(debug_assertions);
    const LAYER_NAMES: [*const c_char; 1] = [to_raw_c_strs!("VK_LAYER_KHRONOS_validation")];

    fn instance_extension_names() -> Vec<*const c_char> {
        let mut instance_extension_names = SurfaceLoader::extension_names();

        if Self::ENABLE_LAYERS {
            instance_extension_names.append(&mut DebugUtilsLoader::extension_names());
        }

        instance_extension_names
    }

    pub fn debug_utils_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
        unsafe extern "system" fn callback(
            _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _p_user_data: *mut c_void,
        ) -> vk::Bool32 {
            println!(
                "Validation layer: {:?}\n",
                CStr::from_ptr((*p_callback_data).p_message)
            );
            false as vk::Bool32
        }

        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(callback))
            .build()
    }

    fn device_extension_names() -> Vec<*const c_char> {
        let device_extension_names = vec![khr::Swapchain::name().as_ptr()];
        device_extension_names
    }
}

///Instance related
impl NewGraphicsCoreAsh {
    fn init_instance(&mut self) {
        let my_version = vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        );
        let app_info = vk::ApplicationInfo::builder()
            .application_name(unsafe { CStr::from_ptr(to_raw_c_strs!("Graphics Core")) }) //spec says utf-8 is available
            .application_version(my_version)
            .engine_name(unsafe { CStr::from_ptr(to_raw_c_strs!("ash")) }) //also utf-8
            .engine_version(my_version) //I wanted ash's own version, but it's impossible and this way also make sense.
            .api_version(vk::API_VERSION_1_0); //currently, no thoughts about specific version

        if !self.is_instance_extensions_supported() {
            panic!("Layers were requested, but not all of them available!");
        }
        let instance_extension_names = Self::instance_extension_names();
        let mut create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extension_names);

        if Self::ENABLE_LAYERS {
            if !self.is_instance_layers_supported() {
                panic!("Layers were requested, but not all of them available!");
            }

            create_info = create_info
                .enabled_layer_names(&Self::LAYER_NAMES)
                .push_next(self.debug_utils_loader.messenger_create_info());
        }

        unsafe {
            self.instance
                .init(self.entry.create_instance(&create_info, None).unwrap());
        }
    }

    fn is_instance_extensions_supported(&self) -> bool {
        let available_extensions_properties = self
            .entry
            .enumerate_instance_extension_properties()
            .unwrap();

        for extension_name in Self::instance_extension_names() {
            let mut supported = false;
            let extension_name = unsafe { CStr::from_ptr(extension_name) };

            for extension_properties in available_extensions_properties.iter() {
                if extension_name
                    == unsafe { CStr::from_ptr(extension_properties.extension_name.as_ptr()) }
                {
                    supported = true;
                    break;
                }
            }

            if !supported {
                return false;
            }
        }

        true
    }

    fn is_instance_layers_supported(&self) -> bool {
        let available_layers_properties = self.entry.enumerate_instance_layer_properties().unwrap();

        for layer_name in Self::LAYER_NAMES {
            let mut supported = false;
            let layer_name = unsafe { CStr::from_ptr(layer_name) };

            for layer_properties in available_layers_properties.iter() {
                if layer_name == unsafe { CStr::from_ptr(layer_properties.layer_name.as_ptr()) } {
                    supported = true;
                    break;
                }
            }

            if !supported {
                return false;
            }
        }

        true
    }

    fn init_debug_utils(&mut self) {
        if !Self::ENABLE_LAYERS {
            return;
        }

        self.debug_utils_loader
            .init(ext::DebugUtils::new(&self.entry, &self.instance));
        self.debug_utils_loader.create_messenger();
    }

    fn init_surface(&mut self) {
        self.surface_loader
            .init(khr::Surface::new(&self.entry, &self.instance));
        self.surface_loader.set_surface(unsafe {
            ash_window::create_surface(
                &self.entry,
                &self.instance,
                globals::APPLICATION.window(), //.as_raw_window_handle(),
                None,
            )
            .expect("Failed to create window surface!")
        });
    }
}

///device related.
impl NewGraphicsCoreAsh {
    fn init_physical_device(&mut self) {
        let physical_devices = unsafe { self.instance.enumerate_physical_devices().unwrap() };

        for physical_device in physical_devices {
            if self.is_device_extensions_supported(physical_device)
                && self.is_format_supported(physical_device)
                && self.is_image_format_supported(physical_device)
                && self.is_memory_properties_supported(physical_device)
            {
                self.physical_device = physical_device;
                break;
            }
        }

        if self.physical_device == vk::PhysicalDevice::null() {
            panic!("Failed to find a proper GPU!")
        }
    }

    fn is_device_extensions_supported(&self, physical_device: vk::PhysicalDevice) -> bool {
        let available_extensions_properties = unsafe {
            self.instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap()
        };

        for extension_name in Self::device_extension_names() {
            let mut supported = false;
            let extension_name = unsafe { CStr::from_ptr(extension_name) };

            for extension_properties in available_extensions_properties.iter() {
                if extension_name
                    == unsafe { CStr::from_ptr(extension_properties.extension_name.as_ptr()) }
                {
                    supported = true;
                    break;
                }
            }

            if !supported {
                return false;
            }
        }

        true
    }

    fn is_format_supported(&self, _physical_device: vk::PhysicalDevice) -> bool {
        true
    }

    fn is_image_format_supported(&self, _physical_device: vk::PhysicalDevice) -> bool {
        true
    }

    fn is_memory_properties_supported(&self, _physical_device: vk::PhysicalDevice) -> bool {
        true
    }
}

///impl destructor
// impl Drop for GraphicsCoreAsh {
//     fn drop(&mut self) {
//         unsafe {
//             if Self::ENABLE_LAYERS {
//                 self.debug_utils_loader
//                     .destroy_debug_utils_messenger(self.debug_utils_messenger_ext, None);
//             }
//             self.instance.destroy_instance(None);
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn graphics() {
        let mut a = NewGraphicsCoreAsh::new();
        a.init();
    }
}
