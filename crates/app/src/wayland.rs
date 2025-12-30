#![allow(unused_doc_comments)]

use crate::{DecorationMode, Decoration};
use std::env;
use core::ffi::c_void;
use renderer::SurfaceBackend;
use log::warn;

//use wayland_client::Connection;
//use wayland_protocols::xdg::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;
//use wayland_protocols::xdg_shell::client::xdg_wm_base;

pub fn supports_blur() -> bool
{
	true
}

/// List of supported DEs/WMs
#[derive(Debug, PartialEq)]
enum DE {
	Hyprland,
	Kde,
	Other,
	Unknown,
}

/// Detect the current DE/WM that the program is beeing executed
fn get_de() -> DE
{
	match env::var("XDG_CURRENT_DESKTOP") {
		Ok(desktop) if desktop.contains("KDE") => DE::Kde,
		Ok(desktop) if desktop.contains("Hyprland") => DE::Hyprland,
		Ok(_) => DE::Other,
		Err(_) => DE::Unknown,
	}
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
	fn new(_title: &str, _width: f64, _height: f64) -> Decoration
	{
		/**
		 * This version will include SSDs and DBusMenu
		 * <https://docs.rs/dbusmenu-glib/latest/dbusmenu_glib/>
		 * On KDE, implement:
		 * - <https://wayland.app/protocols/kde-blur>
		 * - <https://wayland.app/protocols/kde-appmenu>
		 * On Hyprland, implement:
		 * - <https://wayland.app/protocols/hyprland-surface-v1>
		 * Other future (optional) implementations may include:
		 * - popups, notifications, tablet, ext_background_effect_manager_v1
		 */
		return Decoration {
			mode: DecorationMode::ServerSide,
			frame: std::ptr::null_mut() as *const c_void,
			app: std::ptr::null_mut() as *const c_void,
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
