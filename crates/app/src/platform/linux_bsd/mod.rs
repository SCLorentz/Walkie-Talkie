use crate::{
	DecorationMode,
	Decoration,
	WResponse::NotImplementedInCompositor,
	SurfaceBackend,
	NativeDecoration,
	warn
};

use libc::socket;

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {}

impl NativeDecoration for Decoration
{
	fn new(_title: &str, _width: f64, _height: f64) -> Decoration
	{
		return Decoration {
			mode: DecorationMode::ServerSide,
			frame: std::ptr::null_mut() as *const void,
			app: std::ptr::null_mut() as *const void,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn make_view() {}

	fn apply_blur(&self)
	{
		WRequestResult::Fail(NotImplementedInCompositor)
	}
}
