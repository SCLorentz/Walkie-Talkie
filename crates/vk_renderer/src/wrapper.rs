use crate::c_void;

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
