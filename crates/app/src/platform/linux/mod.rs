#![windows_subsystem = "windows"]

#[cfg(feature = "wayland")]
mod wayland;

#[cfg(feature = "wayland")]
pub use wayland::{NativeDecoration, Wrapper};

#[cfg(feature = "x11")]
mod x11;

#[cfg(feature = "x11")]
pub use x11::{NativeDecoration, Wrapper};

/// List of supported DEs/WMs
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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

//use std::env;
//use log::warn;
use crate::{WRequestResult::{Fail, Success}, WResponse, WRequestResult};

/// Detect the current DE/WM that the program is beeing executed
pub fn get_de() -> WRequestResult<DE>
{
	/*let desktop = env::var("XDG_CURRENT_DESKTOP")
		.unwrap_or_else(|_| {
			warn!("missing XDG_CURRENT_DESKTOP");
			String::from("")
		});

	if desktop == "" { return Fail(WResponse::NotImplementedInCompositor) }
	if desktop.contains("KDE") { return Success(DE::Kde) }
	if desktop.contains("Hyprland") { return Success(DE::Hyprland) }*/

	Success(DE::Other)
}
