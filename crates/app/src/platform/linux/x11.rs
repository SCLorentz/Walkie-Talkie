#![allow(unused_doc_comments)]

use crate::{DecorationMode, Decoration};
use crate::platform::linux::DE;

use std::env;
use core::ffi::c_void;
use renderer::SurfaceBackend;
use log::warn;

pub fn supports_blur() -> bool
{
	true
}

/// Detect the current DE/WM that the program is beeing executed
fn get_de() -> DE
{
	let desktop = env::var("XDG_CURRENT_DESKTOP")
		.unwrap_or_else(|_| {
			warn!("missing XDG_CURRENT_DESKTOP");
			String::from("")
		});

	if desktop.contains("XFCE") { return DE::Xfce }

	if desktop == "" { return DE::Unknown }
	DE::Other
}

pub trait XDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: &str, width: f64, height: f64) -> Decoration;
	fn make_view();
	fn apply_blur(&self);
}

impl XDecoration for Decoration
{
	fn new(_title: &str, _width: f64, _height: f64) -> Decoration
	{
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
	}
}
