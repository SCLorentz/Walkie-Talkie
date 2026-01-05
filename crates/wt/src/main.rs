#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/SCLorentz/Walkie-Talkie/issues")]

use app::{App, Event, SurfaceWrapper, Theme, EventHandler};
use vk_renderer::Renderer;
use log::{info, debug};

//use auth::login;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();
	log_panics::init();

	let mut app = App::new(MatrixClient);
	app.set_blur(true);

	let mut window = app.new_window("walkie talkie", (600.0, 500.0));

	let renderer = Renderer::new(window.get_backend())
		.expect("Vulkan inicialization failed");
	window.connect_surface(SurfaceWrapper::new(renderer.surface));

	//login(None);
	app.exec_loop(app_loop);
}

struct MatrixClient;

impl EventHandler for MatrixClient
{
	fn handle_events(e: Event)
	{
		match e {
			Event::CloseRequest => info!("closing now"),
			//Event::WindowResized { window: w, .. } => info!("Resizing window: {:?}", w.title),
			//Event::ThemeChange { new_theme: theme } => info!("changed: {:?}", theme),
			_ => {},
		}
	}
}

fn app_loop()
{
	debug!("app loop");
	std::thread::sleep(std::time::Duration::from_secs(10));
}
