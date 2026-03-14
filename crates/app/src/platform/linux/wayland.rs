#![allow(unused_doc_comments)]
use crate::{
	DecorationMode,
	NativeDecoration,
	Decoration,
	ThemeDefault,
	WResponse::{self, ProtocolNotSuported},
	platform::linux::{DE, get_de},
	void,
	String,
};

use log::debug;
use dirty::{getenv, format, Vec};

#[repr(C)]
struct WaylandMsg {
	object_id: u32,
	opcode: u16,
	size: u16,
	//new_id: u32,
}

impl WaylandMsg {
	pub fn as_raw(&self) -> *const u8
	{
		let mut buf = [0u8; 8];

		buf[0..2].copy_from_slice(&self.object_id.to_le_bytes());
		//buf[2..4].copy_from_slice(&self.object_id.to_le_bytes());
		//buf[4..6].copy_from_slice(&self.object_id.to_le_bytes());

		buf.as_ptr()
	}
}

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub state: *mut void,
	pub surface: *mut void,
	pub socket: *mut void,
}

// wayland_protocols (which include wayland_client) failed to build documentation on version 0.31.12 thks!!
impl NativeDecoration for Decoration
{
	#[allow(unused)]
	fn new(title: String, width: f64, height: f64, theme: ThemeDefault) -> Result<Self, WResponse>
	{
		// https://gaultier.github.io/blog/wayland_from_scratch.html#opening-a-socket
		/// using unwrap_or like this is a bad idea. Use just for now!
		let wayland_display = getenv("WAYLAND_DISPLAY").unwrap_or("wayland-0".as_ptr());
		let runtime_dir = getenv("XDG_RUNTIME_DIR").unwrap_or("/run/user/1000".as_ptr());

		let address: Vec<u8> = format!("{:?}/{:?}", runtime_dir, wayland_display).into_bytes();
		debug!("creating socket on address: {:?}", address);

		let wl_display = dirty::Socket::new(address);

		let get_registry = WaylandMsg { object_id: 1, opcode: 1, size: 12}.as_raw();
		wl_display.write_socket(get_registry.to_raw());

		/*match wl_display.read_socket(buffer) {
			Some(result) => log::debug!("{:?}", result),
			None => log::warn!("no message recived"),
		};*/

		/**
		 * This version will include SSDs and DBusMenu
		 * <https://docs.rs/dbusmenu-glib/latest/dbusmenu_glib/>
		 *
		 * On KDE, implement:
		 * - <https://wayland.app/protocols/kde-blur>
		 * - <https://wayland.app/protocols/kde-appmenu>
		 *
		 * On Hyprland, implement:
		 * - <https://wayland.app/protocols/hyprland-surface-v1>
		 *
		 * Other future (optional) implementations may include:
		 * - popups, notifications, tablet, ext_background_effect_manager_v1
		 */
		let backend = Wrapper {
			state: core::ptr::null_mut::<void>(),
			surface: core::ptr::null_mut::<void>(),
			socket: void::to_handle(wl_display),
		};

		Ok(Decoration {
			mode: DecorationMode::ServerSide,
			frame: core::ptr::null_mut() as *const void, // TODO
			backend,
		})
	}

	fn exit(&self) -> Result<(), WResponse>
	{
		//self.backend.socket.close_socket();
		Ok(())
	}

	fn run(&self) {}

	fn create_app_menu(&self, _app_name: String) -> Result<(), WResponse>
	{ Ok(()) }

	fn apply_blur(&mut self) -> Result<(), WResponse>
	{
		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		let desktop = match get_de() {
			Ok(d) => d,
			Err(_) => DE::Unknown,
		};

		match desktop {
			DE::Hyprland =>
				return Ok(()),
			DE::Kde =>
				return Ok(()),
			_ => {}
		}

		Err(ProtocolNotSuported)
	}
}
