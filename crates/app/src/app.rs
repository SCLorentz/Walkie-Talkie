mod wayland;
mod cocoa;
mod winapi;

use wayland::WaylandWinDecoration;
use cocoa::{CocoaWinDecoration, CocoaDecoration};
use winapi::WindowsWinDecoration;
use log::{debug, warn};
use ash::vk::SurfaceKHR;
use renderer::Renderer;
use std::path::Path;

/// Detect if the current system prefers CSDs or SSDs
/// By default, prefer server side decorations
#[derive(Clone)]
pub enum DecorationMode {
	ClientSide,
	ServerSide,
}

/// Window decoration
pub enum Decoration {
	Apple(CocoaWinDecoration),
	Linux(WaylandWinDecoration),
	Windows(WindowsWinDecoration),
}

impl Decoration {}

/// Window interface
#[allow(unused)]
pub struct Window {
	pub surface: SurfaceKHR,
	pub decoration: Decoration,
	pub cursor: Cursor,
	pub theme: ThemeOp,
	resizable: bool,
	position: (f32, f32),
	title: String,
	surface_size: (f32, f32),
	active: bool,
	blur: bool,
	//id: u32,
}

impl Window {
	/// Create a new window
	pub fn new(title: &'static str) -> Window
	{
		let decoration = Decoration::new();
		let renderer = Renderer::new(decoration.get_view())
			.expect("Vulkan inicialization failed");
		let surface = renderer.surface;

		Window {
			surface,
			decoration,
			cursor: Cursor::get_cursor(),
			surface_size: renderer.get_surface_size(),
			active: false,
			theme: ThemeOp::Light,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
			blur: false
		}
	}

	pub fn is_active(&self) -> bool { self.active }

	pub fn main_loop(&self, code: fn(e: Event))
	{
		debug!("starting main loop");
		loop { code(Window::event()); }
	}

	fn event() -> Event
	{
		Event::Generic
	}
}

#[derive(Debug, Clone)]
pub enum ThemeOp {
	Dark,
	Light,
}

pub trait Theme {
	#[allow(unused)]
	fn set_theme(&mut self, theme: ThemeOp) {}
	fn get_current_theme(&mut self) -> Option<ThemeOp> { None }
}

impl Theme for Window {
	fn set_theme(&mut self, theme: ThemeOp)
		{ self.theme = theme; }

	fn get_current_theme(&mut self) -> Option<ThemeOp>
		{ Some(self.theme.clone()) }
}

/// Default cursor struct
#[allow(unused)]
pub struct Cursor {
	position: (f32, f32),
	texture: Option<String>,
}

impl Cursor {
	pub fn get_cursor() -> Cursor
	{
		use mouse_position::mouse_position::{Mouse};

		let position = Mouse::get_mouse_position();
		let pos = match position {
			Mouse::Position { x, y } => (x as f32, y as f32),
			Mouse::Error => {
				warn!("Couldn't get cursor position. Returning (0.0, 0.0)");
				(0.0, 0.0)
			},
		};

		Cursor { position: pos, texture: None, }
	}

	#[allow(unused)]
	pub fn set_texture(path: &Path) {}
}

/// List of Events
#[derive(Debug, PartialEq)]
pub enum Event {
	MouseIn,
	MouseOut,
	LeftClick,
	RightClick,
	WindowResized,
	WindowMoved,
	ThemeChange,
	CloseRequest,
	RedrawRequest,
	// For now:
	Generic
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vulkan_render() {
		let decoration = Decoration::new();
		let result = Renderer::new(decoration.get_view());
		assert!(result.is_ok());
	}
}
