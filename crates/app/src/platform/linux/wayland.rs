#![allow(unused_doc_comments, unused)]
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

// https://gaultier.github.io/blog/wayland_from_scratch.html
const WAYLAND_DISPLAY_OBJECT_ID:					u32 = 1;
const WAYLAND_WL_REGISTRY_EVENT_GLOBAL:				u16 = 0;
const WAYLAND_SHM_POOL_EVENT_FORMAT:				u16 = 0;
const WAYLAND_WL_BUFFER_EVENT_RELEASE:				u16 = 0;
const WAYLAND_XDG_WM_BASE_EVENT_PING:				u16 = 0;
const WAYLAND_XDG_TOPLEVEL_EVENT_CONFIGURE: 		u16 = 0;
const WAYLAND_XDG_TOPLEVEL_EVENT_CLOSE:				u16 = 1;
const WAYLAND_XDG_SURFACE_EVENT_CONFIGURE:			u16 = 0;
const WAYLAND_WL_DISPLAY_GET_REGISTRY_OPCODE:		u16 = 1;
const WAYLAND_WL_REGISTRY_BIND_OPCODE:				u16 = 0;
const WAYLAND_WL_COMPOSITOR_CREATE_SURFACE_OPCODE: 	u16 = 0;
const WAYLAND_XDG_WM_BASE_PONG_OPCODE:				u16 = 3;
const WAYLAND_XDG_SURFACE_ACK_CONFIGURE_OPCODE:		u16 = 4;
const WAYLAND_WL_SHM_CREATE_POOL_OPCODE:			u16 = 0;
const WAYLAND_XDG_WM_BASE_GET_XDG_SURFACE_OPCODE:	u16 = 2;
const WAYLAND_WL_SHM_POOL_CREATE_BUFFER_OPCODE:		u16 = 0;
const WAYLAND_WL_SURFACE_ATTACH_OPCODE:				u16 = 1;
const WAYLAND_XDG_SURFACE_GET_TOPLEVEL_OPCODE:		u16 = 1;
const WAYLAND_WL_SURFACE_COMMIT_OPCODE:				u16 = 6;
const WAYLAND_WL_DISPLAY_ERROR_EVENT:				u16 = 0;
const WAYLAND_FORMAT_XRGB8888:						u32 = 1;
const WAYLAND_HEADER_SIZE:							u32 = 8;
const COLOR_CHANNELS:								u32 = 4;

/**
 * 4 bytes -> resource id
 * 2 bytes -> method id
 * 2 bytes -> size of the message
 **/
#[repr(C)]
struct GetRegistry {
	object_id:	u32,
	opcode:		u16,
	size:		u16,
	new_id:		u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub state: *mut void,
	pub surface: *mut void,
	pub socket: dirty::Socket,
}

impl NativeDecoration for Decoration
{
	#[allow(unused)]
	fn new(title: String, width: f64, height: f64, theme: ThemeDefault) -> Result<Self, WResponse>
	{
		/*use dirty::ToString;

		// https://gaultier.github.io/blog/wayland_from_scratch.html#opening-a-socket
		/// WARN using unwrap_or like this is a bad idea. Use just for now!
		let wayland_display = getenv("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
		let runtime_dir = getenv("XDG_RUNTIME_DIR").unwrap_or("/run/user/1000".to_string());

		let address= format!("{}/{}", runtime_dir, wayland_display);
		let wl_display = dirty::Socket::connect(&address);

		let get_registry = GetRegistry { object_id: 1, opcode: 1, size: 12, new_id: 2 };
		let get_registry_message: [u8; 4096] = unsafe {
			dirty::as_u8_slice::<GetRegistry, 4096>(get_registry)
		};
		wl_display.send(get_registry_message);

		match wl_display.recv() {
			Some(result) => log::debug!("{:?}", result),
			None => log::warn!("no message recived"),
		};*/

		let backend = Wrapper {
			state: core::ptr::null_mut::<void>(),
			surface: core::ptr::null_mut::<void>(),
			socket: wl_display,
		};

		Ok(Decoration {
			mode: DecorationMode::ServerSide,
			frame: core::ptr::null_mut() as *const void, // TODO
			backend,
		})
	}

	fn exit(&self) -> Result<(), WResponse>
	{
		self.backend.socket.close();
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
