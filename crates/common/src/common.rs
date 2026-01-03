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
	Linux {},
	Headless,
}
