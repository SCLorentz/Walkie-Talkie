/*!
 * Handle with the module exportation here
 * this will keep everything organized up on app.rs
 */

#[cfg(target_os = "macos")]
mod apple;

#[cfg(target_os = "windows")]
mod nt;

#[cfg(any(
	target_os = "linux",
	target_os = "bsd"
))]
mod linux_bsd;

// macOS -------------------
#[cfg(target_os = "macos")]
pub use apple::Wrapper;

// Windows -----------------
#[cfg(target_os = "windows")]
pub use nt::Wrapper;

// Other -------------------
#[cfg(any(
	target_os = "linux",
	target_os = "bsd"
))]
pub use linux_bsd::Wrapper;
