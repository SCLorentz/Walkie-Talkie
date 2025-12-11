// https://wayland.app/protocols/
use std::env;
use std::error::Error;
use crate::DecorationMode;

#[derive(Clone)]
pub struct WaylandWinDecoration {
	pub mode: DecorationMode,
}

#[cfg(target_os = "linux")]
use wayland_client::{
	Display, GlobalManager,
};

#[cfg(target_os = "linux")]
pub fn get_decoration_mode() -> DecorationMode
{
	DecorationMode::ServerSide
}

/// List of supported DEs/WMs
#[derive(Debug)]
pub enum DE {
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
