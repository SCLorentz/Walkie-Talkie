use ash::{
	vk::{self, SurfaceKHR, InstanceCreateFlags, ApplicationInfo	},
	Entry
};
use ash_window::create_surface;
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
use std::{error::Error, ptr::NonNull};
use log::debug;

// WARN this is not recomended
#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;

pub struct Renderer {
	pub surface: SurfaceKHR,
}

impl Renderer {
	/// Creates a new Vulkan render
	pub fn new(view: &NSView) -> Result<Renderer, Box<dyn Error>>
	{
		debug!("Creating new vulkan render");

		let entry = unsafe { Entry::load()? };
		let app_info = ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};

		let extensions: [*const i8; 3] = [
			vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
			vk::KHR_SURFACE_NAME.as_ptr(),
			vk::EXT_METAL_SURFACE_NAME.as_ptr(),
		];

		let validation_layer = std::ffi::CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap();

		let layers = [
			validation_layer.as_ptr()
		];

		let create_info = vk::InstanceCreateInfo {
			p_application_info: &app_info,
			enabled_extension_count: extensions.len() as u32,
			pp_enabled_extension_names: extensions.as_ptr(),
			enabled_layer_count: layers.len() as u32,
			pp_enabled_layer_names: layers.as_ptr(),
			flags: InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
			..Default::default()
		};
		let instance = unsafe { entry.create_instance(&create_info, None)? };

		let display_handle = AppKitDisplayHandle::new();
		let window_handle = AppKitWindowHandle::new(NonNull::from(view).cast());
		// https://www.lunarg.com/wp-content/uploads/2024/03/The-State-of-Vulkan-on-Apple-LunarG-Richard-Wright-03-18-2024.pdf
		let surface = unsafe { create_surface(
			&entry,
			&instance,
			raw_window_handle::RawDisplayHandle::AppKit(display_handle),
			raw_window_handle::RawWindowHandle::AppKit(window_handle),
			None
		)};

		Ok(Renderer {
			surface: surface?
		})
	}

	// for now returns a generic value
	pub fn get_surface_size(&self) -> (f32, f32) { (0.0, 0.0) }
}
