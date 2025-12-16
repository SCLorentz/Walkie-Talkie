mod wayland;
mod cocoa;
mod winapi;

use wayland::WaylandWinDecoration;
use cocoa::{CocoaWinDecoration, CocoaDecoration};
use winapi::WindowsWinDecoration;
use log::{warn, info};
use ash::vk::SurfaceKHR;
use renderer::{Renderer, SurfaceBackend};
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

		let backend =
			SurfaceBackend::MacOS { ns_view: decoration.get_view() };

		let renderer = Renderer::new(backend)
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

#[derive(Debug, Clone, PartialEq)]
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
	/// Modify the current window theme
	/// If alread set as the value provided, it does nothing
	fn set_theme(&mut self, theme: ThemeOp)
		{ self.theme = theme; }

	/// Returns the current global theme of the DE/WM
	fn get_current_theme(&mut self) -> Option<ThemeOp>
		{ Some(self.theme.clone()) }
}

/// List of possible types for the cursor
#[derive(Debug, PartialEq)]
pub enum CursorType {
	Default,
	Pointer,
	TextBox,
	Loading,
	Forbidden,
	Custom(Box<Path>),
}

/// Default cursor struct
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Cursor {
	position: (f32, f32),
	r#type: CursorType,
	visible: bool,
	disabled: bool,
}

// transform this into a trait?
#[allow(dead_code)]
impl Cursor {
	/// Get the cursor object
	pub fn get_cursor() -> Cursor
	{
		Cursor {
			position: Self::get_position(),
			r#type: CursorType::Default,
			visible: true,
			disabled: false,
		}
	}

	/// Returns the current position of the cursor relative to the window
	pub fn get_relative_position() {}

	/// Returns the current position of the cursor
	pub fn get_position() -> (f32, f32)
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

		pos
	}

	/// Modify the cursor position
	pub fn change_position(&mut self, _new_pos: (f32, f32)) {}

	/// Modify the cursor position relative to the window
	pub fn change_relative_position(&mut self, _new_pos: (f32, f32)) {}

	/// Hides the Cursor
	/// If the cursor is already hidden, it does nothing
	pub fn hide(&mut self)
	{
		self.visible = false;
	}

	/// Shows the cursor
	/// If the cursor is already visible, it does nothing
	pub fn show(&mut self)
	{
		self.visible = true;
	}

	/// Locks the cursor in the current place and hides it
	pub fn disable(&mut self)
	{
		info!("disabling cursor");
		self.disabled = true;
		self.hide();
	}

	/// Set the cursor type
	/// For example: `CursorType::Pointer` for click actions or `CursorType::Custom(Path)` for custom textures
	pub fn set_type(&mut self, appearence: CursorType)
	{
		self.r#type = appearence;
	}
}

/// List of Events
#[derive(Debug, PartialEq)]
pub enum Event {
	MouseIn(Cursor),
	MouseOut(Cursor),
	LeftClick,
	RightClick,
	WindowResized,
	WindowMoved,
	ThemeChange(ThemeOp),
	CloseRequest,
	RedrawRequest,
	Focused,
}

// Change this into a separate test inside the Renderer crate
// this test cannot be done on macos
// the idea is to detect any problems on the rendering engine
/*#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_vulkan_render()
	{
		let result = catch_unwind(AssertUnwindSafe(|| {
			use objc2_foundation::MainThreadMarker;
			let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");
			let decoration = Decoration::new(mtm, "test", 600.0, 500.0);
		}));

		assert!(!result.is_err());
	}
}
*/
