#![allow(unused)]
// https://wayland.app/protocols/
use crate::{DecorationMode, Decoration};

#[derive(Clone)]
pub struct WaylandWinDecoration {
	pub mode: DecorationMode,
}

#[cfg(target_os = "linux")]
use wayland_client::{
	Display, GlobalManager,
};

/// Detects if the DE/WM prefers CSD or SSD
#[cfg(target_os = "linux")]
pub fn get_decoration_mode() -> DecorationMode
	{ DecorationMode::ServerSide }

/// List of supported DEs/WMs
#[derive(Debug)]
enum DE {
	Hyprland,
	Kde,
	Gnome,
	Other,
	Unknown,
}

/// Detect the current DE/WM that the program is beeing executed
#[cfg(target_os = "linux")]
pub fn get_de() -> DE
{
	match env::var("XDG_CURRENT_DESKTOP") {
		Ok(desktop) if desktop.contains("KDE") => DE::Kde,
		Ok(desktop) if desktop.contains("GNOME") => DE::Gnome,
		Ok(desktop) if desktop.contains("Hyprland") => DE::Hyprland,
		Ok(desktop) => DE::Other,
		Err(_) => DE::Unknown,
	}
}

#[cfg(not(target_os = "linux"))]
pub trait WaylandDecoration {}

#[cfg(target_os = "linux")]
pub trait WaylandDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new() -> Decoration;
	fn make_view();
	fn get_view(&self);
}

impl WaylandDecoration for Decoration
{
	#[cfg(target_os = "linux")]
	fn new() -> Decoration
	{
		return Decoration::Linux(WaylandWinDecoration {
			mode: wayland::get_decoration_mode(),
		});
	}
}
