//use tokio

use app::Window;
use log::{info, debug};
use std::env;

fn main() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");

	Window::new("my window".to_string());
}
