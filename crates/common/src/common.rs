use core::ffi::c_void;

#[derive(Clone, PartialEq, Debug)]
pub enum SurfaceBackend {
	MacOS {
		ns_view: *mut c_void,
		mtm: *const c_void,
		rect: *const c_void,
	},
	Windows {},
	Linux {
		wayland_view: *mut c_void
	},
	Headless,
}
