#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]

use app::{App, SurfaceWrapper, Theme, ThemeDefault, Color, Event, EventHandler, nb};
use vk_renderer::Renderer;
use nb::Error;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();

	if let Some(path) = dirty::getenv("USER") {
		log::debug!("your user: {:?}", path);
	};

	//let mut theme = App::theme_default(MatrixClient);
	let theme = ThemeDefault {
		blur: true,
		dark: false,
		accent_color: Color::from(255, 255, 255, 255),
		background_color: Color::from(255, 255, 255, 255),
	};

	let mut app = App::new(MatrixClient);
	app.set_theme(theme);

	let mut window = app.new_window("walkie talkie", (600.0, 500.0))
		.expect("window inicialization failed");
	let renderer = Renderer::new(window.get_backend())
		.expect("Vulkan inicialization failed");

	let _ = window.connect_surface(SurfaceWrapper::new(renderer.surface));
	app.init();
}

struct MatrixClient;

impl EventHandler for MatrixClient
{
	fn handle_events(e: Event) -> nb::Result<(), nb::Error<()>>
	{
		match e {
			Event::CloseRequest => log::info!("closing now"),
			Event::WindowResized { window: w, .. } => log::info!("Resizing window: {:?}", w.title),
			Event::ThemeChange { new_theme: theme } => log::info!("changed: {:?}", theme),
			_ => return Err(Error::WouldBlock),
		}
		Ok(())
	}
}
