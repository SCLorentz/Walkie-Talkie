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
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates,
	unused_import_braces,
	unused_qualifications,
	unused_results,
)]
#![allow(clippy::tabs_in_doc_comments)]
#![doc = include_str!("../README.md")]

#[cfg(target_os = "none")]
compile_error!("no bare metal support");

mod platform;
mod events;

pub use events::Event;
use platform::Wrapper;
use log::{warn, info};

//pub use nb;
use dirty::{
	WResponse,
	void,
	String,
	Vec
};

pub use dirty::{SurfaceWrapper as Surface, Color};

/// The default structure to handle and manage apps
#[allow(dead_code)]
pub struct App<H>
where
	H: EventHandler + Send + Sync,
{
	/// List of the program windows
	pub windows: Vec<Window>,
	/// Cursor information
	pub cursor: Cursor,
	theme: ThemeDefault,
	handler: H,
	name: String,
}

/// This is the bridge between system events and the lib events
pub trait EventHandler: Send + Sync
{
	/// handle_events is the only function for the trait and it results a non blocking Event object
	fn handle_events(event: Event); //-> nb::Result<(), nb::Error<()>>;
}

impl<H: EventHandler> App<H>
{
	/// Create a new `App`
	#[must_use]
	pub fn new(handler: H, name: &'static str) -> Self
	{
		let theme = ThemeDefault {
			blur: false,
			dark: false,
			accent_color: Color::from(255, 255, 255, 255),
			background_color: Color::from(255, 255, 255, 255),
			has_title: true,
		};

		Self {
			windows: Vec::new(),
			cursor: Cursor::get_cursor(),
			theme,
			handler,
			name: String::from(name),
		}
	}

	/// Returns the global theme defined as Self::theme_get_default()
	pub fn get_global_theme(&self) -> ThemeDefault
		{ self.theme.clone() }

	/// Modify the current window theme
	/// If alread set as the value provided, it does nothing
	pub fn set_global_theme(&mut self, theme: ThemeDefault)
		{ self.theme = theme }

	/// Creates a new Window element and pushes to the App
	pub fn new_window(
		&mut self,
		title: &'static str,
		size: (f64, f64),
	) -> Result<Window, WResponse>
	{
		let window = Window::new(self.name.clone(), title, self.theme.clone(), size)?;
		self.windows.push(window.clone());
		Ok(window)
	}

	/// init event handler
	pub fn init(&self)
	{
		/*let _event = thread::spawn(move || {
			nb::block!(H::handle_events(Event::Generic)).unwrap();
		});*/

		dirty::Thread::default(event_thread).run();

		#[cfg(target_os = "macos")]
		if let Some(window) = self.windows.first() {
			window.decoration.run();
		};
	}
}

#[unsafe(no_mangle)]
extern "C" fn event_thread(p: *mut void) -> *mut void
{
	log::debug!("creating event thread!");
	p
}

/// Theme struct
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeDefault {
	/// set the alpha value of the window
	pub blur: bool,
	/// default color scheme dark/light
	pub dark: bool,
	/// the default accent color of higlight text, buttons, etc
	pub accent_color: Color,
	/// the background of the window (not of the renderer)
	pub background_color: Color,
	/// Titlebar must be rendered or not
	pub has_title: bool,
}

/// NativeDecoration provides the necessary abstraction used inside the `platform` modules
pub trait NativeDecoration
{
	/// executes the application window
	fn run(&self);
	/// creates a new decoration on the system
	fn new(title: String, width: f64, height: f64, theme: ThemeDefault) -> Result<Self, WResponse> where Self: Sized;
	/// Apply blur to window
	fn apply_blur(&mut self) -> Result<(), WResponse>;
	/// exit handler
	fn exit(&self) -> Result<(), WResponse>;
	/// App Menu Controls
	fn create_app_menu(&self, app_name: String) -> Result<(), WResponse>;
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

/// OS specific. Check platform apple, nt, linux, etc
impl Decoration {}

/// Window interface
#[derive(Clone, PartialEq, Debug)]
pub struct Window {
	/// Window title
	pub title: String,
	/// The graphical backend (on our case, vulkan)
	pub surface: Option<Surface>,
	/// The native window frame
	decoration: Decoration,
	resizable: bool,
	position: (f32, f32),
	active: bool,
	theme: ThemeDefault,
}

#[forbid(unsafe_code)]
impl Window
{
	/// Get system specific window backend (for renderer)
	#[must_use]
	pub fn get_backend(&self) -> *mut void
		{ void::to_handle(self.decoration.backend.clone()) }

	/// Connects a specified vulkan surface with the current window
	pub fn connect_surface(&mut self, surface: Surface) -> Result<(), WResponse>
	{
		if !self.has_surface() {
			self.surface = Some(surface);
			return Ok(());
		}
		warn!("this window is already connected to a surface!");
		info!("to connect to another surface, please remove the current one");
		Err(WResponse::ChannelInUse)
	}

	/// Returns if window does have a surface or not
	#[must_use]
	pub fn has_surface(&self) -> bool
		{ self.surface.is_some() }

	/// Detects if the window is focused
	#[must_use]
	pub fn is_active(&self) -> bool { self.active }

	/// Changes the `window.resizable` argument to a specific bool val
	pub fn resizable(&mut self, arg: bool) { self.resizable = arg }
}

trait PrivateWindow {
	fn new(app_name: String, title: &'static str, theme: ThemeDefault, size: (f64, f64)) ->
		Result<Window, WResponse>;
}

impl PrivateWindow for Window {
	fn new(
		app_name: String,
		title: &'static str,
		theme: ThemeDefault,
		size: (f64, f64)
	) -> Result<Self, WResponse>
	{
		#[allow(unused_mut)]
		let mut decoration = match Decoration::new(
			String::from(title),
			size.0,
			size.1,
			theme.clone()
		) {
			Ok(v) => v,
			Err(_) => return Err(WResponse::UnexpectedError),
		};

		let _menu = decoration.create_app_menu(app_name);

		if theme.blur
		&& let Err(response) = decoration.apply_blur()
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
	/*/// Custom cursor sprite
	 Custom(Box<Path>)*/
}

/// Default cursor struct
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
	position: (f64, f64),
	mode: CursorType,
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
			mode: CursorType::Default,
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
		let _err: objc2_core_graphics::CGError =
			objc2_core_graphics::CGDisplayHideCursor(0);

		self.visible = false;
	}

	/// Shows the cursor
	/// If the cursor is already visible, it does nothing
	pub fn show(&mut self)
	{
		#[cfg(target_os = "macos")]
		let _err: objc2_core_graphics::CGError =
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
	pub fn set_type(&mut self, mode: CursorType)
		{ self.mode = mode; }

	/// Detects if the cursor is visible or not
	#[must_use]
	pub fn is_visible(&self) -> bool
		{ self.visible }
}
