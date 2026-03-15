#![allow(unused_doc_comments, unused)]
use crate::{
	DecorationMode,
	NativeDecoration,
	Decoration,
	ThemeDefault,
	WResponse::{self, ProtocolNotSuported},
	platform::linux::{DE, get_de},
	void,
	String,
};

use log::debug;
use dirty::{format, Vec};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WindowSurface {
	display: *mut void,
	registry: *mut void,
	registry_listener: *mut void,
	surface: *mut void,
	toplevel: *mut void,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub surface: WindowSurface,
}

unsafe extern "C" {
	pub(crate) fn request_wl_surface() -> WindowSurface;
	pub(crate) fn request_wl_disconnect(display: *mut void);
	pub(crate) fn loop_wl_event(display: *mut void);
}

impl NativeDecoration for Decoration
{
	fn new(title: String, width: f64, height: f64, theme: ThemeDefault) -> Result<Self, WResponse>
	{
		let wl_display = unsafe { request_wl_surface() };
		let frame = wl_display.toplevel;

		let backend = Wrapper {
			surface: wl_display,
		};

		Ok(Decoration {
			mode: DecorationMode::ServerSide,
			frame,
			backend,
		})
	}

	fn exit(&self) -> Result<(), WResponse>
	{
		unsafe { request_wl_disconnect(self.backend.surface.display) };
		Ok(())
	}

	fn run(&self)
	{
		unsafe { loop_wl_event(self.backend.surface.display) };
	}

	fn create_app_menu(&self, _app_name: String) -> Result<(), WResponse>
		{ Ok(()) }

	fn apply_blur(&mut self) -> Result<(), WResponse>
	{
		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		let desktop = match get_de() {
			Ok(d) => d,
			Err(_) => DE::Unknown,
		};

		match desktop {
			DE::Hyprland =>
				return Ok(()),
			DE::Kde =>
				return Ok(()),
			_ => {}
		}

		Err(ProtocolNotSuported)
	}
}
