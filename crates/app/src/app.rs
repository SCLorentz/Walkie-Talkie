mod wayland;
mod cocoa;
mod winapi;

#[allow(unused)]
use wayland::WaylandWinDecoration;
use cocoa::CocoaDecoration;
#[allow(unused)]
use winapi::WindowsWinDecoration;
use log::{warn, info, debug};
use ash::vk::SurfaceKHR;
use renderer::{Renderer, SurfaceBackend};
use std::path::Path;
use core::ffi::c_void;

/// List of Events
#[derive(Debug, PartialEq)]
pub enum Event {
	MouseIn { cursor: Cursor, window: Window },
	MouseOut { cursor: Cursor, window: Window },
	LeftClick { cursor: Cursor, window: Window },
	RightClick { cursor: Cursor, window: Window },
	WindowResized { window: Window },
	WindowMoved { window: Window },
	ThemeChange { new_theme: ThemeOp },
	RedrawRequest { window: Window },
	Focused { window: Window },
	CloseRequest,
}

#[allow(dead_code)]
pub struct App {
	/// list of active windows
	pub windows: Vec<Window>,
	cursor: Cursor,
	theme: ThemeOp,
}

impl App {
	pub fn new() -> Self
	{
		Self {
			windows: Vec::new(),
			theme: ThemeOp::Light,
			cursor: Cursor::get_cursor(),
		}
	}

	/// Creates a new Window element and pushes to the App
	pub fn new_window(&mut self, title: &'static str)
	{
		let window = Window::new(title);
		self.windows.push(window);
	}

	/// The execution loop to be executed on the program.
	/// Can be used to handle with events.
	pub fn exec_loop(&self, run: fn(e: Option<Event>))
	{
		std::thread::spawn(move || {
			loop { run(App::event()); }
		});

		// TODO: be careful with that
		#[cfg(target_os = "macos")]
		self.windows[0].decoration.run();
	}

	fn event() -> Option<Event>
	{
		None
	}
}

/// Detect if the current system prefers CSDs or SSDs
/// By default, prefer server side decorations
#[derive(Clone, PartialEq, Debug)]
pub enum DecorationMode {
	ClientSide,
	ServerSide,
}

#[derive(Debug, PartialEq, Clone)]
struct Decoration {
	frame: *const c_void,
	backend: SurfaceBackend,
	mode: DecorationMode,
	app: *const c_void,
}

/// Decoration specific values
/// This is empty because each OS implements their own traits
impl Decoration {}

/// Window interface
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Window {
	/// Window title
	pub title: String,
	/// The vulkan render surface
	pub surface: SurfaceKHR,
	/// The native window frame
	decoration: Decoration,
	resizable: bool,
	position: (f32, f32),
	surface_size: (f32, f32),
	active: bool,
	blur: bool,
	//id: u32,
}

impl Window {
	/// Create a new window
	pub fn new(title: &'static str) -> Self
	{
		use objc2_foundation::MainThreadMarker;
		let mtm = MainThreadMarker::new()
			.expect("Process expected to be executed on the Main Thread!");

		let decoration = Decoration::new(mtm, title, 600.0, 500.0);
		let view = decoration.get_view();

		#[cfg(target_os = "macos")]
		let backend = SurfaceBackend::MacOS { ns_view: view };

		let renderer = Renderer::new(backend.clone())
			.expect("Vulkan inicialization failed");
		let surface = renderer.surface;

		Window {
			decoration,
			surface,
			surface_size: renderer.get_surface_size(),
			active: false,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
			blur: false
		}
	}

	/// Detects if the window is focused
	pub fn is_active(&self) -> bool { self.active }

	#[allow(unused)]
	pub fn close_window(&self) {}
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

impl Theme for App {
	/// Modify the current window theme
	/// If alread set as the value provided, it does nothing
	fn set_theme(&mut self, theme: ThemeOp)
		{ self.theme = theme; }

	/// Returns the current global theme of the DE/WM
	fn get_current_theme(&mut self) -> Option<ThemeOp>
		{ Some(self.theme.clone()) }
}

/// List of possible types for the cursor
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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
	pub fn get_cursor() -> Self
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

#[ctor::ctor]
fn logger() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");
}
