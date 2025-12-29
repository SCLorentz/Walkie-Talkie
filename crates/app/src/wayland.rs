#![allow(unused_doc_comments)]

use crate::{DecorationMode, Decoration};
use std::env;
use core::ffi::c_void;
use renderer::SurfaceBackend;
use log::warn;

//use wayland_protocols::xdg::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;

/// Detects if the DE/WM prefers CSD or SSD
pub fn get_decoration_mode() -> DecorationMode
{
	if get_de() == DE::Gnome {
		warn!("GNOME DE detected! Client side decoration not implemented");
		return DecorationMode::ClientSide
	}

	//let toplevel_decoration = ZxdgDecorationManagerV1::get_toplevel_decoration();
	/**
	 * <https://wayland.app/protocols/xdg-decoration-unstable-v1>
	 * Every major compositor (except of course GNOME) has implemented the XDG_DECORATION protocol
	 * get_toplevel_decoration(id: new_id<zxdg_toplevel_decoration_v1>, toplevel: object<xdg_toplevel>)
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
fn get_de() -> DE
{
	match env::var("XDG_CURRENT_DESKTOP") {
		Ok(desktop) if desktop.contains("KDE") => DE::Kde,
		Ok(desktop) if desktop.contains("GNOME") => DE::Gnome,
		Ok(desktop) if desktop.contains("Hyprland") => DE::Hyprland,
		Ok(_) => DE::Other,
		Err(_) => DE::Unknown,
	}
}

pub fn supports_blur() -> bool
{
	true
}

pub trait WaylandDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: &str, width: f64, height: f64) -> Decoration;
	fn make_view();
	fn apply_blur(&self);
}

impl WaylandDecoration for Decoration
{
	#[allow(unused)]
	fn new(title: &str, width: f64, height: f64) -> Decoration
	{
		/**
		 * On KDE, implement:
		 * - <https://wayland.app/protocols/kde-blur>
		 * - <https://wayland.app/protocols/kde-appmenu>
		 * On Hyprland, implement:
		 * - <https://wayland.app/protocols/hyprland-surface-v1>
		 * Other future (optional) implementations may include:
		 * - popups, notifications, tablet, ext_background_effect_manager_v1
		 */
		return Decoration {
			mode: get_decoration_mode(),
			frame: 1 as *const c_void,
			app: 1 as *const c_void,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn make_view() {}

	fn apply_blur(&self)
	{
		if !supports_blur() {
			warn!("couldn't set window blur, desktop doesn't implement protocol");
			return
		}

		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		if get_de() == DE::Hyprland
			{ return }
	}
}
