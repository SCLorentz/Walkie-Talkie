//use tokio

use app::Window;
use log::info;

fn main() {
	log_panics::init();
	info!("starting program");

	Window::new();
}
