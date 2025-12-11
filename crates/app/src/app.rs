mod wayland;
mod cocoa;
mod winapi;

#[cfg(target_os = "linux")]
use wayland::get_de;

#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;

#[cfg(target_os = "macos")]
use objc2::{
	MainThreadOnly,
	MainThreadMarker,
	rc::Retained,
};

#[cfg(target_os = "macos")]
use objc2_foundation::{NSRect, NSPoint, NSSize};

use wayland::WaylandWinDecoration;
use cocoa::CocoaWinDecoration;
use winapi::WindowsWinDecoration;
use log::debug;
use ash::vk::SurfaceKHR;
#[allow(unused)]
use renderer::Renderer;

/// Detect if the current system prefers CSDs or SSDs
/// By default, prefer server side decorations
#[derive(Clone)]
pub enum DecorationMode {
	ClientSide,
	ServerSide,
}

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
			view: Self::make_view(),
		});
	}

	pub fn get_view(&self) -> Option<&NSView> {
		match self {
			Decoration::Apple(dec) => Some(&dec.view),
			_ => None,
		}
	}

	fn make_view() -> Retained<NSView> {
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");

		let origin = NSPoint::new(10.0, -2.3);
		let size = NSSize::new(5.0, 0.0);
		let rect = NSRect::new(origin, size);

		unsafe {
			NSView::initWithFrame(NSView::alloc(mtm), rect)
		}
	}
}

/// Window interface
#[allow(unused)]
pub struct Window {
	pub surface: Option<SurfaceKHR>,
	pub decoration: Decoration,
	//id: u32,
	surface_size: (f32, f32),
	active: bool,
	pub cursor: Cursor,
	pub theme: ThemeOp,
	resizable: bool,
	position: (f32, f32),
	title: String,
	//blur --> compositor has support for blur? Is it enabled?
}

impl Window {
	pub fn new(title: &'static str) -> Window
	{
		let decoration = Decoration::new();
		let renderer = Renderer::new(decoration.get_view().expect("No view"));

		Window {
			surface: Some(renderer.unwrap().surface.expect("No surface found")), // for now this is will panic
			//surface: None,
			decoration,
			cursor: Cursor { position: (0.0, 0.0), texture: None, },
			surface_size: (0.0, 0.0),
			active: false,
			theme: ThemeOp::Light,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
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
	//Custom(T) <-- could be useful for linux
}

pub trait Theme {
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
pub struct Cursor {
	position: (f32, f32),
	texture: Option<String>,
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
	Generic // remove later
}
