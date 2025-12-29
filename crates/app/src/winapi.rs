#![allow(unused)]
use crate::{DecorationMode, Decoration};

#[cfg(target_os = "windows")]
use windows::{
	core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
	Win32::UI::WindowsAndMessaging::*,
};

#[cfg(target_os = "windows")]
#[derive(Clone, PartialEq, Debug)]
pub struct WindowsWinDecoration {
	pub mode: DecorationMode,
}

#[cfg(target_os = "windows")]
pub trait WindowsDecoration {
	fn new() -> Decoration;
	fn make_view();
	fn get_view(&self);
}

#[cfg(target_os = "windows")]
impl WindowsDecoration for Decoration
{
	fn new() -> Decoration
	{
		return Decoration {
			mode: DecorationMode::ServerSide,
		};
	}

	fn make_view() {}

	fn get_view() {}
}
