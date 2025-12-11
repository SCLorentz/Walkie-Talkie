#![allow(unused)]
use crate::{DecorationMode, Decoration};

#[cfg(target_os = "windows")]
use windows::{
	core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
	Win32::UI::WindowsAndMessaging::*,
};

#[derive(Clone)]
pub struct WindowsWinDecoration {
	pub mode: DecorationMode,
}

pub trait WindowsDecoration {
	#[cfg(target_os = "windows")]
	fn new() -> Decoration;

	#[cfg(target_os = "windows")]
	#[allow(unused)]
	fn make_view();

	#[cfg(target_os = "windows")]
	fn get_view(&self);
}

impl WindowsDecoration for Decoration
{
	#[cfg(target_os = "windows")]
	fn new() -> Decoration {
		return Decoration::Windows(WindowsWinDecoration {
			mode: DecorationMode::ServerSide,
		});
	}
}
