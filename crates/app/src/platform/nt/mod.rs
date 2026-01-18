#![windows_subsystem = "windows"]
#![doc = include_str!("./README.md")]
// https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/core_app/src/main.rs
// TODO: make a VM and test this on windows (fix all problems)
// TODO: make this work as well on reactOS

use crate::{
	DecorationMode,
	Decoration,
	void,
	String,
	WRequestResult
};

mod system;
use system::NtQuerySystemInformation;

extern crate alloc;

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {}

pub trait NativeDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: String, _width: f64, _height: f64) -> WRequestResult<Self> where Self: core::marker::Sized;
	fn make_view();
	fn apply_blur(&self) -> WRequestResult<()>;
}

impl NativeDecoration for Decoration {
	fn new(title: String, _width: f64, _height: f64) -> WRequestResult<Self>
	{
		unsafe {
			let mut len: u32 = 0;
			let status =
				NtQuerySystemInformation(5, core::ptr::null_mut(), 0, &mut len);

			dirty::write!("status: {:#x}", status);
		}

		return Decoration {
			mode: DecorationMode::ServerSide,
			frame: core::ptr::null_mut() as *const void,
			backend: Wrapper {},
		};
	}

	fn make_view() { todo!() }

	fn apply_blur(&self) -> WRequestResult<()> { todo!() }
}
