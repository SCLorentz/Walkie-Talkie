#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]
use app::{Event, EventHandler, Window};

struct MatrixClient;

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

fn main()
{
	use app::App;
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();

	let mut app = App::new(MatrixClient, "Walkie Talkie");
	let mut theme = app.get_global_theme();
			theme.blur = true;
			theme.has_title = true;
	app.set_global_theme(theme);

	if let Ok(mut window) = app.new_window("walkie talkie", (600.0, 500.0))
	{
		let backend = app.get_backend(&window);
		let renderer = vk_renderer::Renderer::new(backend)
			.expect("Vulkan inicialization failed");
		let _ = app.connect_surface(&mut window, renderer.get_surface());
	};

	let _ = app.new_window("window 2", (500.0, 500.0));

	app.init();
}

/*
 * maybe useful:
 * https://developer.apple.com/documentation/accelerate/vimage-library
 */
