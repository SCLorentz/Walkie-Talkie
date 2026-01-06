# App

With the app crate is possible to create multi-platform, native apps and windows.

With the app structure you can create windows, connect them to a rendering pipeline and manage events. The app structure is divided in three main modules, macOS, windows and linux. Linux itself has other variations to support x11 and native GNOME builds. By default it uses SSD or the native decoration method, except on GNOME which doesn't support it.

## Examples

```rust
use app::{App, Event, SurfaceWrapper};
use renderer::Renderer;
use debug::info;

struct MyApp;

impl EventHandler for MyApp
{
	fn handle_events(e: Event)
	{
		match e {
			Event::CloseRequest => info!("closing now"),
			Event::WindowResized { window: w, .. } => info!("Resizing window: {:?}", w.title),
			Event::ThemeChange { new_theme: theme } => info!("changed: {:?}", theme),
			_ => {},
		}
	}
}

fn main() {
	let mut app = App::new(MyApp);
	app.set_blur(true);

	let mut window = app.new_window("walkie talkie", (600.0, 500.0));
	let renderer = Renderer::new(&mut window.get_backend())
		.expect("Vulkan inicialization failed");
	window.connect_surface(SurfaceWrapper::new(renderer.surface));

	app.init();
}
```

hide the cursor

```rust
app.cursor.hide();
```

Check if the cursor is hidden

```rust
app.cursor.is_visible();
```
