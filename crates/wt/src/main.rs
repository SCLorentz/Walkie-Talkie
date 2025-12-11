use app::{Window, Theme, Event};
use log::{info, debug};

fn main() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");

	let mut app = Window::new("my window");
	app.main_loop(|e| {
		if e != Event::Generic {
			info!("event: {:?}", e);
		}
	});

	let Some(t) = app.get_current_theme() else { return };
	debug!("current theme set as {:?}", t);
}
