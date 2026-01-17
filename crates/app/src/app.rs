#![no_std]
#![allow(
	clippy::tabs_in_doc_comments,
	unused_doc_comments
)]
#![deny(unreachable_code)]
#![doc = include_str!("../README.md")]

// Redox is compatible with the linux ABI, minimum ajustments needed
#[cfg(target_os = "redox")]
compile_error!("redox not supported yet");

#[cfg(target_os = "none")]
compile_error!("no bare metal support");

mod platform;
mod events;

pub use events::Event;
use platform::{NativeDecoration, Wrapper};
use log::{warn, info};
use dirty::{
	WRequestResult::{self, Fail, Success},
	WResponse,
	Color,
	void,
	String,
	Box
};

pub use dirty::SurfaceWrapper;

#[allow(dead_code)]
pub struct App//<H>
//where
//	H: EventHandler + Send + Sync,
{
	pub windows: Box<[Window]>,
	pub cursor: Cursor,
	theme: ThemeDefault,
	//handler: H,
}

impl Default for App {
	fn default() -> Self
		{ Self::new() }
}

//pub trait EventHandler: Send + Sync
//	{ fn handle_events(event: Event); }

//impl<H: EventHandler> App<H>
impl App
{
	pub fn new() -> Self
	{
		Self {
			windows: Box::new([]),
			theme: Self::theme_default(),
			cursor: Cursor::get_cursor(),
			//handler,
		}
	}

	/// Creates a new Window element and pushes to the App
	pub fn new_window(&mut self, title: &'static str, size: (f64, f64)) -> Window
	{
		let window = Window::new(title, self.theme.clone(), size);
		match self.windows.len() {
			0 => self.windows = Box::new([window.clone()]),
			len => self.windows[len] = window.clone(),
		};

		window
	}

	/**
	 * In the future, merge the target macos and linux exec_loop() into one single
	 */
	pub fn init(&self)
	{
		// event thread
		// Use non-blocking I/O here to wait for the events
		// this will start at 0,0% CPU and at some point it will escalate to 100%
		// im so stupid...
		/*std::thread::spawn(move || {
			loop {
				H::handle_events(Event::Generic);
			};
		});*/


		#[cfg(target_os = "macos")]
		self.windows[0].decoration.run();
		//unsafe { (*self.windows[0]).decoration.run() };
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
pub struct Decoration {
	frame: *const void,
	backend: Wrapper,
	mode: DecorationMode,
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
	pub surface: Option<SurfaceWrapper>,
	/// The native window frame
	decoration: Decoration,
	resizable: bool,
	position: (f32, f32),
	active: bool,
	theme: ThemeDefault,
	//id: u32,
}

impl Window
{
	/// Create a new window
	pub fn new(title: &'static str, theme: ThemeDefault, size: (f64, f64)) -> Self
	{
		#[allow(unused_mut)]
		let mut decoration = Decoration::new(String::from(title), size.0, size.1);

		if theme.blur
		&& let Fail(response) = decoration.apply_blur()
			{ warn!("{:?}", response) };

		Window {
			decoration,
			surface: None,
			active: false,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
			theme,
		}
	}

	pub fn get_backend(&self) -> *mut void
		{ void::to_handle(self.decoration.backend.clone()) }

	pub fn some_surface(&self) -> bool
		{ self.surface.is_some() }

	pub fn connect_surface(&mut self, surface: SurfaceWrapper) -> WRequestResult<()>
	{
		if !self.has_surface() {
			self.surface = Some(surface);
			return Success(());
		}
		warn!("this window is already connected to a surface!");
		info!("to connect to another surface, please remove the current one");
		Fail(WResponse::ChannelInUse)
	}

	pub fn has_surface(&self) -> bool
		{ self.surface.is_some() }

	/// Detects if the window is focused
	pub fn is_active(&self) -> bool { self.active }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeDefault {
	blur: bool,
	dark: bool,
	accent_color: Color,
}

pub trait Theme {
	fn set_theme(&mut self, theme: ThemeDefault);
	fn get_current_theme(&mut self) -> WRequestResult<ThemeDefault>;
	fn theme_default() -> ThemeDefault;
	fn set_blur(&mut self, blur: bool);
}

impl Theme for App
{
	/// Modify the current window theme
	/// If alread set as the value provided, it does nothing
	fn set_theme(&mut self, theme: ThemeDefault)
		{ self.theme = theme; }

	/// Returns the current global theme of the DE/WM
	fn get_current_theme(&mut self) -> WRequestResult<ThemeDefault>
		{ Success(self.theme.clone()) }

	fn theme_default() -> ThemeDefault
	{
		ThemeDefault {
			blur: false,
			dark: false,
			accent_color: Color::from(255, 255, 255, 255),
		}
	}

	/// Get the current theme and change the blur value to 'true'
	fn set_blur(&mut self, blur: bool)
		{ self.theme.blur = blur; }
}

/// List of possible types for the cursor
#[derive(Debug, PartialEq, Clone)]
pub enum CursorType {
	Default,
	Pointer,
	TextBox,
	Loading,
	Forbidden,
	//Custom(Box<Path>),
}

/// Default cursor struct
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
	position: (f64, f64),
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
	pub fn get_position() -> (f64, f64)
	{
		#[cfg(target_os = "macos")]
		let pos = objc2_app_kit::NSEvent::mouseLocation();

		#[cfg(not(target_os = "macos"))]
		let (x, y) = (0.0, 0.0);

		#[cfg(target_os = "macos")]
		let (x, y) = (pos.x, pos.y);

		(x, y)
	}

	/// Modify the cursor position
	pub fn change_position(&mut self, _new_pos: (f64, f64)) {}

	/// Modify the cursor position relative to the window
	pub fn change_relative_position(&mut self, _new_pos: (f64, f64)) {}

	/// Hides the Cursor
	/// If the cursor is already hidden, it does nothing
	pub fn hide(&mut self)
	{
		#[cfg(target_os = "macos")]
		objc2_core_graphics::CGDisplayHideCursor(0);
		self.visible = false;
	}

	/// Shows the cursor
	/// If the cursor is already visible, it does nothing
	pub fn show(&mut self)
	{
		#[cfg(target_os = "macos")]
		objc2_core_graphics::CGDisplayShowCursor(0);
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
		{ self.r#type = appearence; }

	pub fn is_visible(&self) -> bool
		{ self.visible }
}
