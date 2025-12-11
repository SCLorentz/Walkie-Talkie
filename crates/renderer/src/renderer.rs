#[allow(unused)]
use ash::{vk::{self, SurfaceKHR}, Entry};
#[allow(unused)]
use ash_window::create_surface;
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
use std::error::Error;
use std::ptr::NonNull;

use objc2_app_kit::NSView;

#[allow(unused)]
use log::{info, warn, debug};

pub struct Renderer {
	//pub context
	pub surface: SurfaceKHR,
}

impl Renderer {
	pub fn new(view: &NSView) -> Result<Renderer, Box<dyn Error>>
	{
		debug!("new renderer");

		let entry = unsafe { Entry::load()? };
		let app_info = vk::ApplicationInfo {
			api_version: vk::API_VERSION_1_1,
			..Default::default()
		};
		let create_info = vk::InstanceCreateInfo {
			p_application_info: &app_info,
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

	fn init_pipeline(&mut self) -> Result<(), Box<dyn Error>>
	{
		Ok(())
	}
}

/*#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_renderer() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}*/
