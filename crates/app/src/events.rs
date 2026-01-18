use crate::{ThemeDefault, Window, Cursor};

/// List of Events
#[derive(Debug, PartialEq)]
pub enum Event {
	/// Mouse enters the window
	MouseIn {
		/// The cursor position
		cursor: Cursor,
		/// The specified window
		window: Window,
	},
	/// Mouse leaves the window
	MouseOut {
		/// The cursor position
		cursor: Cursor,
		/// The specified window
		window: Window
	},
	/// Element clicked with left button
	LeftClick {
		/// The cursor position
		cursor: Cursor,
		/// The specified window
		window: Window
	},
	/// Element clicked with right button
	RightClick {
		/// The cursor position
		cursor: Cursor,
		/// The specified window
		window: Window
	},
	/// Window resized (action by user)
	WindowResized {
		/// The specified window
		window: Window,
		/// The new window size
		new_size: (f64, f64)
	},
	/// Window moved (action by user)
	WindowMoved {
		/// The specified window
		window: Window,
		/// The new window position
		new_positon: (f64, f64)
	},
	/// User changed global theme
	ThemeChange {
		/// The new theme
		new_theme: ThemeDefault
	},
	/// Redraw frame
	RedrawRequest {
		/// The specified window
		window: Window
	},
	/// The user focused the window
	Focused {
		/// The specified window
		window: Window
	},
	/// User wants to leave
	CloseRequest,
	/// Temporary argument to handle with the impossibility of implementation (todo)
	Generic,
}
