# App

With the app crate is possible to create multi-platform, native apps and windows.

With the app structure you can create windows, connect them to a rendering pipeline and manage events. The app structure is divided in three main modules, macOS, windows and linux (with both wayland and x11 support). By default it uses SSD or the native decoration method with fallback to CSD when unsuported.

## Example

```rust
struct MyApp;

impl app::EventHandler for MyApp
{
	fn handle_events(e: app::Event)
	{
		use app::Event;
		match e {
			Event::CloseRequest => log::info!("closing now"),
			Event::WindowResized { window: w, .. } => log::info!("Resizing window: {:?}", w.title),
			Event::OsThemeChange { new_theme: theme } => log::info!("changed: {theme:?}"),
			_ => {}
		}
	}
}

fn main() {
	use app::App;

	let mut app = App::new(MatrixClient, "Walkie Talkie");
	let mut theme = app.get_global_theme();
			theme.blur = true;
			theme.has_title = true;
	app.set_global_theme(theme);

	if let Ok(mut window) = app.new_window("walkie talkie", (600.0, 500.0))
	{
		let renderer = vk_renderer::Renderer::new(window.get_backend())
			.expect("Vulkan inicialization failed");
		let _ = window.connect_surface(renderer.get_surface());
	}

	let _ = app.new_window("window 2", (500.0, 500.0));

	app.init();
}
```
