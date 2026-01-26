use crate::{
	DecorationMode,
	Decoration,
	platform::linux::{DE, get_de},
	WResponse::NotImplementedInCompositor,
	SurfaceBackend,
	NativeDecoration,
	warn
};

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
