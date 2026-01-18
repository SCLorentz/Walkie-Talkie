#![no_std]
#![deny(
	deprecated,
	rust_2018_idioms,
	clippy::shadow_unrelated,
	unreachable_code,
	unused_imports,
	unused_variables,
	unsafe_op_in_unsafe_fn,
	clippy::unwrap_used,
	clippy::expect_used,
	clippy::shadow_reuse,
	clippy::shadow_same,
	clippy::dbg_macro,
	clippy::print_stdout,
	clippy::print_stderr,
	clippy::panic,
	clippy::indexing_slicing,
	clippy::arithmetic_side_effects,
	clippy::float_arithmetic,
	clippy::unwrap_in_result,
	clippy::exit,
	clippy::wildcard_imports,
	missing_docs,
	clippy::all,
)]
#![allow(
	clippy::tabs_in_doc_comments,
	unused_doc_comments,
	clippy::missing_errors_doc
)]
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
use core::error::Error;

/// The default structure to handle and manage apps
#[allow(dead_code)]
pub struct App//<H>
//where
//	H: EventHandler + Send + Sync,
{
	/// List of the program windows
	pub windows: Box<[Window]>,
	/// Cursor information
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
	/// Create a new `App`
	#[must_use]
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
	pub fn new_window(
		&mut self,
		title: &'static str,
		size: (f64, f64)
	) -> Result<Window, Box<dyn Error>>
	{
		let window = Window::new(title, self.theme.clone(), size)?;
		match self.windows.len() {
			0 => self.windows = Box::new([window.clone()]),
			len => {
				let Some(some_window) =
					self.windows.get_mut(len) else { return Err(Box::from("no window was found")) };
				*some_window = window.clone();
			},
		}

		Ok(window)
	}

	/// init event handler
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
		let Some(window) = self.windows.first() else {
			log::warn!("no windows found on self.windows");
			return
		};
		window.decoration.run();
	}
}

/// Detect if the current system prefers CSDs or SSDs
/// By default, prefer server side decorations
#[derive(Clone, PartialEq, Debug)]
pub enum DecorationMode {
	/// Render the window decorations on the compositor
	ClientSide,
	/// Render the window decorations in the window surface
	ServerSide,
}

/// Default struct for window Decorations
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
	pub fn new(
		title: &'static str,
		theme: ThemeDefault,
		size: (f64, f64)
	) -> Result<Self, Box<dyn Error>>
	{
		let mut decoration = match Decoration::new(String::from(title), size.0, size.1) {
			Success(v) => v,
			Fail(_) => return Err(Box::from("something went wrong creating decoration")),
		};

		if theme.blur
		&& let Fail(response) = decoration.apply_blur()
			{ warn!("{response:?}") }

		Ok(Window {
			decoration,
			surface: None,
			active: false,
			resizable: true,
			position: (0.0, 0.0),
			title: String::from(title),
			theme,
		})
	}

	/// Get system specific window backend (for renderer)
	#[must_use]
	pub fn get_backend(&self) -> *mut void
		{ void::to_handle(self.decoration.backend.clone()) }

	/// Connects a specified vulkan surface with the current window
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

	/// Returns if window does have a surface or not
	#[must_use]
	pub fn has_surface(&self) -> bool
		{ self.surface.is_some() }

	/// Detects if the window is focused
	#[must_use]
	pub fn is_active(&self) -> bool { self.active }
}

/// Theme struct
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeDefault {
	blur: bool,
	dark: bool,
	accent_color: Color,
}

/// Default Trait functions for windows
pub trait Theme {
	/// Set window specific theme
	fn set_theme(&mut self, theme: ThemeDefault);
	/// Get window specific theme
	fn get_current_theme(&mut self) -> WRequestResult<ThemeDefault>;
	/// Get the global theme
	fn theme_default() -> ThemeDefault;
	/// Set the blur effect on specified window
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
	/// The generic arrow cursor
	Default,
	/// The pointer cursor
	Pointer,
	/// Text selection cursor
	TextBox,
	/// Loading cursor
	Loading,
	/// Forbidden cursor
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
	#[must_use]
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
	#[must_use]
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

	/// Detects if the cursor is visible or not
	#[must_use]
	pub fn is_visible(&self) -> bool
		{ self.visible }
}
