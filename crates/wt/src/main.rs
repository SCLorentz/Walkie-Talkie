use app::{App, Event};
use log::{info, debug};

#[allow(unused)]
use auth::login;
//use url::Url;

fn main() {
	simple_logger::SimpleLogger::new()
		.init()
		.unwrap();
	log_panics::init();
	debug!("starting program");

	let mut app = App::new(true);
	app.new_window("walkie talkie");

	//login(None);
	app.exec_loop(app_loop);
}

fn app_loop(event: Option<Event>)
{
	match event {
		Some(Event::CloseRequest) => info!("closing now"),
		Some(Event::WindowResized { window: w, .. }) => info!("Resizing window: {:?}", w.title),
		Some(Event::ThemeChange { new_theme: theme }) => info!("changed: {:?}", theme),
		Some(other) => info!("event: {:?}", other),
		None => {}
	}
}
