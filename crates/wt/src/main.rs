//use tokio

use std::sync::mpsc;
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, ExecutableCommand, cursor};
use crossterm::terminal;
use std::thread;

use app::{Window, Theme};
use log::{info, debug};

fn main() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");

	let mut app = Window::new("my window");
	/*app.main_loop(|| {
		return String::from("hello world")
	});*/

	let Some(t) = app.get_current_theme() else { return };
	debug!("current theme set as {:?}", t);
}
