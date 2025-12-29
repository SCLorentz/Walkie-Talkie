#![allow(unused)]
use crate::{DecorationMode, Decoration};

use windows::{
	core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
	Win32::UI::WindowsAndMessaging::*,
};

pub trait WindowsDecoration {
	fn new() -> Decoration;
	fn make_view();
	fn set_blur();
}

impl WindowsDecoration for Decoration
{
	fn new() -> Decoration
	{
		return Decoration {
			mode: DecorationMode::ServerSide,
		};
	}

	fn make_view() {}
	fn apply_blur(&self) {}
}
