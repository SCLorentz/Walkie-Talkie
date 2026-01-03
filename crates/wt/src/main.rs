#![doc = include_str!("../README.md")]

use app::{App, Event, SurfaceWrapper};
use renderer::Renderer;
use log::{info, debug};

//use auth::login;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();
	log_panics::init();
	debug!("starting program");

	let mut app = App::new(true);
	let mut window = app.new_window("walkie talkie");

	let renderer = Renderer::new(&mut window.get_backend())
		.expect("Vulkan inicialization failed");
	window.connect_surface(SurfaceWrapper::new(renderer.surface));

	//login(None);
	app.exec_loop(app_loop);
}

fn app_loop(event: Option<Event>)
{
	match event {
		Some(Event::CloseRequest) => info!("closing now"),
		Some(Event::WindowResized { window: w, .. }) => info!("Resizing window: {:?}", w.title),
		Some(Event::ThemeChange { new_theme: theme }) => info!("changed: {:?}", theme),
		Some(other) => info!("event: {:?}", other),
		None => {}
	}
}
