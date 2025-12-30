/**
 * Handle with the module exportation here
 * this will keep everything organized up on app.rs
 */

#[cfg(all(target_os = "linux", not(feature = "gnome")))]
mod wayland;

#[cfg(all(target_os = "linux", feature = "gnome"))]
mod gnome;

#[cfg(target_os = "macos")]
mod cocoa;

#[cfg(target_os = "windows")]
mod winapi;

// Linux --------------------
#[cfg(all(target_os = "linux", not(feature = "gnome")))]
pub use wayland::WaylandDecoration as NativeDecoration;

#[cfg(all(target_os = "linux", feature = "gnome"))]
pub use gnome::GnomeDecoration as NativeDecoration;

// macOS -------------------
#[cfg(target_os = "macos")]
pub use cocoa::CocoaDecoration as NativeDecoration;

// Windows -----------------
#[cfg(target_os = "windows")]
pub use winapi::WindowsDecoration as NativeDecoration;
