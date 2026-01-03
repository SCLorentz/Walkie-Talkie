use core::ffi::c_void;

#[derive(Clone, PartialEq, Debug)]
pub enum SurfaceBackend {
	MacOS {
		ns_view: *mut c_void,		// NSView
		mtm: *const c_void,			// MainThreadMaker
		rect: *const c_void,		// NSRect
		app: *const c_void,			// NSApplication
	},
	Windows {},
	/// <https://github.com/Smithay/wayland-rs/blob/master/wayland-client/examples/simple_window.rs>
	Linux {
		state: *mut c_void
	},
	Headless,
}

#[cfg(target_os = "macos")]
use objc2::{rc::Retained, Message};

#[cfg(target_os = "macos")]
/// In a simple way, this will turn any objc `Retained<T>` into a c_void
pub fn macos_generic<T>(ptr: &Retained<T>)
	-> *mut c_void where T: Message
{
	let ptr: *mut T = Retained::<T>::as_ptr(&ptr) as *mut T;
	ptr.cast()
}
