use ash::{vk::{self, SurfaceKHR}, Entry};
use ash_window::create_surface;
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
use std::error::Error;
use std::ptr::NonNull;

#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;

pub struct Renderer {
	pub surface: SurfaceKHR,
}

impl Renderer {
	/// Creates a new Vulkan renderer
	pub fn new(view: &NSView) -> Result<Renderer, Box<dyn Error>>
	{
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

	// for now returns a generic value
	pub fn get_surface_size(&self) -> (f32, f32) { (0.0, 0.0) }
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
