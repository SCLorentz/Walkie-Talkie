//use std::error::Error;
mod wayland;
mod cocoa;
mod winapi;

#[cfg(target_os = "linux")]
use wayland::get_de;

use wayland::WaylandWinDecoration;
use cocoa::CocoaWinDecoration;
use winapi::WindowsWinDecoration;
use log::{info, debug};
use ash::vk::SurfaceKHR;
use renderer::Renderer;

/// Detect if the current system prefers CSDs or SSDs
/// By default, prefer server side decorations
#[derive(Clone)]
pub enum DecorationMode {
	ClientSide,
	ServerSide,
}

#[derive(Clone)]
enum Decoration {
	Apple(CocoaWinDecoration),
	Linux(WaylandWinDecoration),
	Windows(WindowsWinDecoration),
}

impl Decoration
{
	pub fn new() -> Decoration
	{
		#[cfg(target_os = "linux")]
		return Decoration::Linux(WaylandWinDecoration {
			mode: wayland::get_decoration_mode(),
		});

		#[cfg(target_os = "macos")]
		return Decoration::Apple(CocoaWinDecoration {
			mode: DecorationMode::ServerSide,
		});
	}
}

/// Window interface
#[allow(unused)]
pub struct Window {
	pub surface: Option<SurfaceKHR>,
	pub decoration: Decoration,
	//id: u32,
	//surface_size
	active: bool,
	//cursor
	pub theme: ThemeOp,
	resizable: bool,
	position: (f32, f32),
	title: String,
	//blur --> compositor has support for blur? Is it enabled?
}

#[allow(unused)]
pub enum Event {
	MouseIn,
	MouseOut,
	LeftClick,
	RightClick,
	WindowResized,
	WindowMoved,
	ThemeChange,
	CloseRequest,
	// For now:
	Generic // remove later
}

impl Window {
	pub fn new(title: &'static str) -> Window
	{
		let decoration = Decoration::new();
		//let renderer = Renderer::new(decoration);

		Window {
			//surface: Some(renderer.surface.unwrap()), // for now this will panic
			surface: None,
			decoration,
			active: false,
			theme: ThemeOp::Light,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
		}
	}

	/*pub fn main_loop(&self, code: fn() -> String)
	{
		std::thread::spawn(move || loop { code() });
	}*/

	pub fn event() -> Event
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
	fn set_theme(&mut self, theme: ThemeOp) {}
	fn get_current_theme(&mut self) -> Option<ThemeOp> { None }
}

impl Theme for Window {
	fn set_theme(&mut self, theme: ThemeOp)
	{
		self.theme = theme;
	}

	fn get_current_theme(&mut self) -> Option<ThemeOp>
	{
		Some(self.theme.clone())
	}
}
