#![allow(unused_doc_comments, unused)]
use crate::{
	DecorationMode,
	NativeDecoration,
	Decoration,
	ThemeDefault,
	platform::linux::{DE, get_de},
	void,
	String,
	WResponse::{self, ProtocolNotSuported},
};

use dirty::getenv;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Wrapper {
	pub state:   *mut void,
	pub surface: *mut void,
	pub socket:  *mut void,
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
	pub wl_surface: *mut void,
	pub wl_display: *mut void,
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
		let state = unsafe { request_wl_surface() };
		let frame = state.toplevel;

		let backend = Wrapper {
			wl_surface: state.surface,
			wl_display: state.display,
		};

		let decoration = Self {
			mode: DecorationMode::ServerSide,
			frame,
			backend,
		};

		Ok(decoration)
	}

	fn exit(&self) -> Result<(), WResponse>
	{
		unsafe { request_wl_disconnect(self.backend.wl_display) };
		Ok(())
	}

	fn run(&self)
	{
		unsafe { loop_wl_event(self.backend.wl_display) };
	}

	fn create_app_menu(&self, _app_name: String) -> Result<(), WResponse>
		{ Ok(()) }

	fn apply_blur(&mut self) -> Result<(), WResponse>
	{
		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		let desktop = get_de().map_or(DE::Unknown, |d| d);

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
