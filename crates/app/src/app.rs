// Linux imports
#[cfg(target_os = "linux")]
mod wayland;

#[cfg(target_os = "linux")]
use wayland::get_de;

// MacOS imports
#[cfg(target_os = "macos")]
mod cocoa;

#[cfg(target_os = "macos")]
use cocoa::CocoaWinDecoration;

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
struct CfgDecoration {
	mode: DecorationMode,
}

#[derive(Clone)]
struct Decoration {
	//wayland: Option<T>,
	cocoa: Option<CocoaWinDecoration>,
	//winapi: Option<T>,
}

impl Decoration
{
	pub fn new() -> Decoration
	{
		debug!("creating new window decoration");
		//debug!("DE {:?}", get_de());
		Decoration {
			cocoa: None
		}
	}
}

pub enum Theme {
	Dark,
	Light,
}

/// Window interface
pub struct Window {
	pub surface: Option<SurfaceKHR>,
	pub decoration: Decoration,
	//id: u32,
	//surface_size
	active: bool,
	//cursor
	pub theme: Theme,
	resizable: bool,
	position: (f32, f32),
	title: String,
	//blur --> compositor has support for blur? Is it enabled?
}

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
	pub fn new(title: String) -> Window
	{
		let decoration = Decoration::new();
		//let renderer = Renderer::new();

		Window {
			//surface: Some(renderer.surface.unwrap()), // for now this will panic
			surface: None,
			decoration,
			active: false,
			theme: Theme::Light,
			resizable: true,
			position: (0.0, 0.0),
			title,
		}
	}

	// use tokio here?
	pub fn main_loop()
	{
		debug!("main loop");
	}

	pub fn event() -> Event
	{
		debug!("handle with events here");
		Event::Generic
	}
}
