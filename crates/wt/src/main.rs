use app::{Window, Theme};
use log::{debug, info};

fn main() {
	env_logger::init();
	log_panics::init();
	debug!("starting program");

	let mut app = Window::new("my window");
	let Some(t) = app.get_current_theme() else { return };
	debug!("current theme set as {:?}", t);

	app.exec_loop(|e| {
		if e.is_some() {
			info!("event: {:?}", e);
		}
	});
}

/*#[cfg(test)]
mod tests {
	use super::*;

	#[app::test]
	fn create_app() {
		let mut app = Window::new("test");
		let theme = app.get_current_theme();

		assert!(theme.is_some());
		//assert!(result.is_ok());
	}
}
*/
