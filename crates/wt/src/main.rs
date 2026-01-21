#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]

use app::{App, SurfaceWrapper, Theme};
use vk_renderer::Renderer;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();

	if let Some(path) = dirty::getenv("USER") {
		log::debug!("your user: {:?}", path);
	};

	let mut app = App::default();
	app.set_blur(true);

	let mut window = app.new_window("walkie talkie", (600.0, 500.0))
		.expect("missing window");
	let renderer = Renderer::new(window.get_backend())
		.expect("Vulkan inicialization failed");

	let _ = window.connect_surface(SurfaceWrapper::new(renderer.surface));

	app.init();
}
