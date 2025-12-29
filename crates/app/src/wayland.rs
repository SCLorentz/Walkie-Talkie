#![allow(unused_doc_comments)]

use crate::{DecorationMode, Decoration};
use std::env;
use core::ffi::c_void;
use renderer::SurfaceBackend;

#[derive(Clone, PartialEq, Debug)]
pub struct WaylandWinDecoration {
	pub mode: DecorationMode,
}

//#[cfg(target_os = "linux")]
//use wayland_protocols::wp::alpha_modifier::v1::client::*;

/// Detects if the DE/WM prefers CSD or SSD
#[cfg(target_os = "linux")]
pub fn get_decoration_mode() -> DecorationMode
{
	let desktop = get_de();
	if desktop == DE::Gnome {
		return DecorationMode::ClientSide
	}
	/**
	 * https://wayland.app/protocols/xdg-decoration-unstable-v1
	 * Every major compositor (except of course GNOME) has implemented the XDG_DECORATION protocol
	 */
	DecorationMode::ServerSide
}

/// List of supported DEs/WMs
#[derive(Debug, PartialEq)]
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

#[cfg(target_os = "linux")]
pub trait WaylandDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: &str, width: f64, height: f64) -> Decoration;
	fn make_view();
	fn get_view(&self);
}

#[cfg(target_os = "linux")]
impl WaylandDecoration for Decoration
{
	fn new(title: &str, width: f64, height: f64) -> Decoration
	{
		/**
		 * On KDE, implement:
		 * - https://wayland.app/protocols/kde-blur
		 * - https://wayland.app/protocols/kde-appmenu
		 * On Hyprland, implement:
		 * - https://wayland.app/protocols/hyprland-surface-v1
		 */
		return Decoration {
			mode: get_decoration_mode(),
			frame: 1 as *const c_void,
			app: 1 as *const c_void,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn make_view() {}
	fn get_view(&self) {}
}
