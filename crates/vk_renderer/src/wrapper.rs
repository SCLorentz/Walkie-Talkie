use crate::void;

#[cfg(target_os = "linux")]
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub state: *mut void,
	pub surface: *mut void,
}

#[cfg(target_os = "macos")]
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub ns_view: *mut void,		// NSView
	pub rect: *const void,		// NSRect
	pub app: *const void,		// NSApplication
}

#[cfg(target_os = "windows")]
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {}
