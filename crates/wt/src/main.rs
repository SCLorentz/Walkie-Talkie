use app::{App, Event};
use log::info;

fn main() {
	let mut app = App::new(true);
	app.new_window("walkie talkie");
	app.exec_loop(app_loop);
}

fn app_loop(event: Option<Event>)
{
	match event {
		Some(Event::CloseRequest) => info!("closing now"),
		Some(Event::WindowResized { window: w }) => info!("Resizing window: {:?}", w.title),
		Some(Event::ThemeChange { new_theme: theme }) => info!("changed: {:?}", theme),
		Some(other) => info!("event: {:?}", other),
		None => {}
	}
}
