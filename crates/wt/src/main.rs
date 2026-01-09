#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]

use app::{App, SurfaceWrapper, Theme};
//use core::arch::asm;
//use log::debug;
use vk_renderer::Renderer;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();

	let mut app = App::new();
	app.set_blur(true);

	let mut window = app.new_window("walkie talkie", (600.0, 500.0));

	let renderer = Renderer::new(window.get_backend())
		.expect("Vulkan inicialization failed");
	window.connect_surface(SurfaceWrapper::new(renderer.surface));

	app.init();
}
