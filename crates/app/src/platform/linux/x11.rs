#![allow(unused_doc_comments)]

use crate::{
	DecorationMode,
	Decoration,
	platform::linux::DE,
	WResponse::NotImplementedInCompositor,
	SurfaceBackend,
	warn
};

use core::ffi::c_void;
use super::shared::get_de;

pub trait XDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: &str, width: f64, height: f64) -> Decoration;
	fn make_view();
	fn apply_blur(&self);
}

impl XDecoration for Decoration
{
	fn new(_title: &str, _width: f64, _height: f64) -> Decoration
	{
		return Decoration {
			mode: DecorationMode::ServerSide,
			frame: std::ptr::null_mut() as *const c_void,
			app: std::ptr::null_mut() as *const c_void,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn make_view() {}

	fn apply_blur(&self)
	{
		WRequestResult::Fail(NotImplementedInCompositor)
	}
}
