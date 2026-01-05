use core::ffi::c_void;
use crate::{from_handle, SurfaceBackend};

#[cfg(target_os = "linux")]
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub state: *mut c_void
}

#[cfg(target_os = "macos")]
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub ns_view: *mut c_void,		// NSView
	pub rect: *const c_void,		// NSRect
	pub app: *const c_void,			// NSApplication
}

impl SurfaceBackend for Wrapper {
	fn get_surface(backend: *mut c_void) -> *mut c_void
	{
		let backend: Self = unsafe { from_handle(backend) };

		#[cfg(target_os = "macos")]
		return backend.ns_view as *mut c_void;

		#[cfg(target_os = "linux")]
		todo!();
	}
}
