use std::env;
use crate::{warn, platform::linux::DE};

/// Detect the current DE/WM that the program is beeing executed
pub fn get_de() -> DE
{
	let desktop = env::var("XDG_CURRENT_DESKTOP")
		.unwrap_or_else(|_| {
			warn!("missing XDG_CURRENT_DESKTOP");
			String::from("")
		});

	if desktop == "" { return DE::Unknown }
	if desktop.contains("KDE") { return DE::Kde }
	if desktop.contains("Hyprland") { return DE::Hyprland }

	DE::Other
}
