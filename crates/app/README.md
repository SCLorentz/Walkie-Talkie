# App

The app structure is divided in three main modules, macOS, windows and linux. Linux itself has other variations to support x11 and native GNOME builds.

## Example

```rust
fn main() {
	let mut app = App::new(true);

	app.new_window("my application");
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
