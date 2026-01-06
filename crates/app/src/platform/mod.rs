/**
 * Handle with the module exportation here
 * this will keep everything organized up on app.rs
 */

#[cfg(target_os = "macos")]
mod apple;

#[cfg(target_os = "windows")]
mod nt;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "bsd")]
mod bsd;

// macOS -------------------
#[cfg(target_os = "macos")]
pub use apple::{NativeDecoration, Wrapper};

// Windows -----------------
#[cfg(target_os = "windows")]
pub use nt::{NativeDecoration, Wrapper};

// Linux -------------------
#[cfg(target_os = "linux")]
pub use linux::{NativeDecoration, Wrapper};

// BSD ---------------------
#[cfg(target_os = "bsd")]
pub use bsd::{NativeDecoration, Wrapper};
