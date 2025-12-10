//mod wayland;
mod cocoa;
use cocoa::CocoaWinDecoration;

use log::info;
use ash::vk::SurfaceKHR;
use renderer::Renderer;

#[derive(Clone)]
struct Decoration {
	//wayland: Option<T>,
	cocoa: Option<CocoaWinDecoration>,
	//winapi: Option<T>,
}

impl Decoration
{
	pub fn new() -> Decoration
	{
		info!("creating new window decor");
		Decoration {
			cocoa: None
		}
	}
}

pub struct Window {
	pub surface: Option<SurfaceKHR>,
	pub decoration: Decoration,
	//id: u32,
	//surface_size
	//active
	//cursor
	//theme
	//resizable
	//position
	//title
	//blur
}


impl Window {
	pub fn new() -> Window
	{
		let decoration = Decoration::new();
		let renderer = Renderer::new();

		Window {
			surface: None,
			decoration,
		}
	}
}
