use crate::{
	DecorationMode,
	Decoration,
	WRequestResult,
	WResponse::ProtocolNotSuported,
	platform::linux::{DE, get_de},
	void,
	String,
	WRequestResult::Success
};

pub trait NativeDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: String, width: f64, height: f64) -> WRequestResult<Self> where Self: core::marker::Sized;
	/// apply blur to window
	fn apply_blur(&self) -> WRequestResult<()>;
	/// exit handler
	#[allow(unused)]
	fn exit(&self);
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
	fn new(_title: String, _width: f64, _height: f64) -> WRequestResult<Self>
	{
		let address = b"wayland-0";
		let socket = dirty::Socket::new(address);
		socket.write_socket(b"hello world");

		let buffer: u8 = b"";
		match socket.read_socket(buffer) {
			Some(result) => log::debug!("{:?}", result),
			None => log::warn!("no message recived"),
		};
		//socket.close_socket();

		/**
		 * This version will include SSDs and DBusMenu
		 * <https://docs.rs/dbusmenu-glib/latest/dbusmenu_glib/>
		 * On KDE, implement:
		 * - <https://wayland.app/protocols/kde-blur>
		 * - <https://wayland.app/protocols/kde-appmenu>
		 * On Hyprland, implement:
		 * - <https://wayland.app/protocols/hyprland-surface-v1>
		 * Other future (optional) implementations may include:
		 * - popups, notifications, tablet, ext_background_effect_manager_v1
		 */
		let backend = Wrapper {
			state: core::ptr::null_mut::<void>(),
			surface: core::ptr::null_mut::<void>(),
			socket,
		};

		Success(Decoration {
			mode: DecorationMode::ServerSide,
			frame: core::ptr::null_mut() as *const void, // TODO
			backend,
		})
	}

	fn exit(&self)
	{
		self.socket.close_socket();
		todo!();
	}

	fn apply_blur(&self) -> WRequestResult<()>
	{
		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		let desktop = match get_de() {
			dirty::WRequestResult::Success(d) => d,
			dirty::WRequestResult::Fail(_) => DE::Unknown,
		};

		match desktop {
			DE::Hyprland =>
				return WRequestResult::Success(()),
			_ => {}
		}

		WRequestResult::Fail(ProtocolNotSuported)
	}
}
