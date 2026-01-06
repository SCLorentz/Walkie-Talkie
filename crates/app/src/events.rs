use crate::{ThemeDefault, Window, Cursor};

/// List of Events
#[derive(Debug, PartialEq)]
pub enum Event {
	MouseIn {
		cursor: Cursor,
		window: Window,
	},
	MouseOut {
		cursor: Cursor,
		window: Window
	},
	LeftClick {
		cursor: Cursor,
		window: Window
	},
	RightClick {
		cursor: Cursor,
		window: Window
	},
	WindowResized {
		window: Window,
		new_size: (f64, f64)
	},
	WindowMoved {
		window: Window,
		new_positon: (f64, f64)
	},
	ThemeChange {
		new_theme: ThemeDefault
	},
	RedrawRequest {
		window: Window
	},
	Focused {
		window: Window
	},
	CloseRequest,
	Generic,
}
