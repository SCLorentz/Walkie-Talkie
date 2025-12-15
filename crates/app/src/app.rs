mod wayland;
mod cocoa;
mod winapi;

use wayland::WaylandWinDecoration;
use cocoa::{CocoaWinDecoration, CocoaDecoration};
use winapi::WindowsWinDecoration;
use log::warn;
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
#[allow(dead_code)]
pub struct Window {
	/// The vulkan render surface
	pub surface: SurfaceKHR,
	/// The native window frame
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
		use objc2_foundation::MainThreadMarker;
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");
		let decoration = Decoration::new(mtm, title, 600.0, 500.0);

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

	/// Detects if the window is focused
	pub fn is_active(&self) -> bool { self.active }

	/// The execution loop to be executed on the program.
	/// Can be used to handle with events.
	pub fn exec_loop(&self, run: fn(e: Option<Event>))
	{
		std::thread::spawn(move || {
			loop { run(Window::event()); }
		});

		#[cfg(target_os = "macos")]
		self.decoration.run();
	}

	fn event() -> Option<Event>
	{
		None
	}
}

#[derive(Debug, Clone)]
pub enum ThemeOp {
	Dark,
	Light,
}

#[allow(dead_code)]
pub trait Theme {
	fn set_theme(&mut self, _theme: ThemeOp) {}
	fn get_current_theme(&mut self) -> Option<ThemeOp> { None }
}

impl Theme for Window {
	fn set_theme(&mut self, theme: ThemeOp)
		{ self.theme = theme; }

	fn get_current_theme(&mut self) -> Option<ThemeOp>
		{ Some(self.theme.clone()) }
}

/// Default cursor struct
#[allow(dead_code)]
pub struct Cursor {
	position: (f32, f32),
	texture: Option<String>,
}

// transform this into a trait?
#[allow(dead_code)]
impl Cursor {
	/// Get the current position of the cursor
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

	/// Get the current position of the cursor relative to the window
	pub fn relative_position() {}

	pub fn change_pos(_new_pos: (f32, f32)) {}

	pub fn hide() {}

	pub fn show() {}

	pub fn disable() {}

	pub fn set_texture(_path: &Path) {}
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
	Focused,
}
