# App

The app structure is divided in three main modules, macOS, windows and linux. Linux itself has other variations to support x11 and native GNOME builds.

## Example

```rust
use app::{App, Event, SurfaceWrapper};
use renderer::Renderer;
use debug::info;

fn main() {
	let mut app = App::new(true);
	let mut window = app.new_window("walkie talkie");

	let renderer = Renderer::new(&mut window.get_backend())
		.expect("Vulkan inicialization failed");
	window.connect_surface(SurfaceWrapper::new(renderer.surface));

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
```
