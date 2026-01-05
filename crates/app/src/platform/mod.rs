/**
 * Handle with the module exportation here
 * this will keep everything organized up on app.rs
 */

#[cfg(target_os = "macos")]
mod cocoa;

#[cfg(target_os = "windows")]
mod winapi;

#[cfg(target_os = "linux")]
mod linux;

// macOS -------------------
#[cfg(target_os = "macos")]
pub use cocoa::{
	CocoaDecoration as NativeDecoration,
	Wrapper
};

// Windows -----------------
#[cfg(target_os = "windows")]
pub use winapi::WindowsDecoration as NativeDecoration;

// Linux -------------------

#[cfg(target_os = "linux")]
pub use linux::{NativeDecoration, Wrapper};
