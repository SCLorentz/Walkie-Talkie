//use std::panic::{catch_unwind, AssertUnwindSafe};
use app::{Window, Event};
use log::{debug, info};

fn main() {
	let _app = Window::new("my window")
		.exec_loop(window_loop);
}

fn window_loop(event: Option<Event>)
{
	match event {
		Some(Event::CloseRequest) => info!("closing now"),
		Some(Event::WindowResized) => info!("Resizing window"),
		Some(Event::ThemeChange(theme)) => info!("changed: {:?}", theme),
		Some(other) => info!("event: {:?}", other),
		None => {}
	}
}

#[ctor::ctor]
fn logger() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");
}

/*#[cfg(test)]
mod tests {
	use super::*;

	/// Just a simple example (forceful fail)
	/// On macOS this will fail because it's not executed on the main thread
	#[cfg(target_os = "macos")]
	#[test]
	fn thread_error_macos()
	{
		let result = catch_unwind(AssertUnwindSafe(|| {
			Window::new("test");
		}));

		assert!(result.is_err());
	}

	//#[app::test]
	//fn test_vulkan_render()
}*/
