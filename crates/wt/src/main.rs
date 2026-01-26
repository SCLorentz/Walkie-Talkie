#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]

use app::{App, Event, EventHandler};

#[allow(unused)]
struct MatrixClient {
	field: bool
}

impl EventHandler for MatrixClient
{
	fn handle_events(e: Event)
	{
		match e {
			Event::CloseRequest => log::info!("closing now"),
			Event::WindowResized { window: w, .. } => log::info!("Resizing window: {:?}", w.title),
			Event::OsThemeChange { new_theme: theme } => log::info!("changed: {:?}", theme),
			_ => {}
		}
	}
}

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();

	let matrix_client = MatrixClient { field: true };

	let mut app = App::new(matrix_client);
	let mut theme = app.get_global_theme();
			theme.blur = true;
			theme.has_title = true;
	app.set_global_theme(theme);

	if let Ok(mut window) = app.new_window("walkie talkie", (600.0, 500.0))
	{
		let renderer = vk_renderer::Renderer::new(window.get_backend())
			.expect("Vulkan inicialization failed");
		let _ = window.connect_surface(renderer.get_surface());
	};

	let _ = app.new_window("window 2", (500.0, 500.0));

	app.init();
}
