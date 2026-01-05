//pub use crate::{DecorationMode, Decoration};

#[cfg(feature = "wayland")]
mod wayland;

#[cfg(feature = "gnome")]
mod gnome;

#[cfg(feature = "x11")]
mod x11;

#[cfg(feature = "wayland")]
pub use wayland::WaylandDecoration as NativeDecoration;
pub use wayland::LinuxWrapper as Wrapper;

#[cfg(feature = "gnome")]
pub use gnome::GnomeDecoration as NativeDecoration;

#[cfg(feature = "x11")]
pub use x11::XDecoration as NativeDecoration;

/// List of supported DEs/WMs
#[derive(Debug, PartialEq)]
pub enum DE {
	Kde,
	Hyprland,
	Sway,
	Xfce,
	/// Not officially ported
	Other,
	/// Couldn't detect
	Unknown,
}
