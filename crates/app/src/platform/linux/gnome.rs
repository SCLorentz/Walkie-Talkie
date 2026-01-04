#![allow(unused_doc_comments)]
// if you are reading this, you have been gnomed

use crate::{DecorationMode, Decoration, WRequestResult, WResponse};
use crate::platform::linux::DE;

use std::env;
use core::ffi::c_void;
use crate::SurfaceBackend;
use log::warn;

pub trait GnomeDecoration
{
	/// Creates a CSD decoration for the GNOME window
	fn new(title: &str, width: f64, height: f64) -> Decoration;
	fn make_view();
	fn apply_blur(&self) -> WindowRequestResult<()>;
	fn draw_frame_decor(&self);
}

impl GnomeDecoration for Decoration
{
	fn new(title: &str, width: f64, height: f64) -> Decoration
	{
		return Decoration {
			mode: DecorationMode::ClientSide,
			frame: 1 as *const c_void,
			app: 1 as *const c_void,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn draw_frame_decor(&self) {}

	fn make_view() {}

	fn apply_blur(&self) -> WRequestResult<()>
	{
		use WRequestResult::Fail;
		use WResponse::BinarySpecificLimitation;

		warn!("Sorry, this version of the executable doesn't offer support for blur");
		Fail(BinarySpecificLimitation);
	}
}
