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

/*use objc2_app_kit::NSView;
use objc2_foundation::{NSRect, NSPoint, NSSize};
use objc2::{msg_send_id, rc::Id, MainThreadOnly};

unsafe fn create_view() -> Id<NSView> {
	let frame = NSRect::new(
		NSPoint::new(0., 0.),
		NSSize::new(100., 100.),
	);

	let view: Id<NSView> = msg_send_id![NSView::alloc(), initWithFrame: frame];
	view
}*/

pub struct Renderer {
	//pub context
	pub surface: Option<SurfaceKHR>, // later: remove the Option<> tag
}

impl Renderer {
	pub fn new(view: &NSView) -> Result<Renderer, Box<dyn Error>>
	{
		debug!("new renderer");

		let entry = unsafe { Entry::load()? };
		let app_info = vk::ApplicationInfo {
			api_version: vk::make_api_version(0, 1, 0, 0),
			..Default::default()
		};
		let create_info = vk::InstanceCreateInfo {
			p_application_info: &app_info,
			..Default::default()
		};
		let instance = unsafe { entry.create_instance(&create_info, None)? };

		let display_handle = AppKitDisplayHandle::new();
		let window_handle = AppKitWindowHandle::new(NonNull::from(view).cast());
		let surface = unsafe { create_surface(
			&entry,
			&instance,
			raw_window_handle::RawDisplayHandle::AppKit(display_handle),
			raw_window_handle::RawWindowHandle::AppKit(window_handle),
			None
		)};

		Ok(Renderer {
			surface: Some(surface?)
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
