use app::{Window, Theme};
use log::debug;

fn main() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");

	let mut app = Window::new("my window");
	let Some(t) = app.get_current_theme() else { return };
	debug!("current theme set as {:?}", t);

	app.run();
}
