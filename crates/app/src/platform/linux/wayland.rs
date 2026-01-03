#![allow(unused_doc_comments)]

use crate::{DecorationMode, Decoration, SurfaceBackend};
use crate::platform::linux::DE;

use std::{fs::File, os::unix::io::AsFd};
use std::env;
use core::ffi::c_void;
use log::warn;

use wayland_client::{
	delegate_noop,
	protocol::{
		wl_buffer, wl_compositor, wl_keyboard, wl_registry, wl_seat, wl_shm, wl_shm_pool,
		wl_surface,
	},
	Connection, Dispatch, QueueHandle, WEnum, EventQueue
};

use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

struct State {
	running: bool,
	base_surface: *mut c_void,	// wl_surface::WlSurface
	buffer:	*mut c_void,		// wl_buffer::WlBuffer
	wm_base: *mut c_void,		// xdg_wm_base::XdgWmBase
	xdg_surface: *const c_void,
	configured: bool,
}

impl State {
	fn init_xdg_surface(&mut self, qh: &QueueHandle<State>)
	{
		let wm_base = self.wm_base as *mut xdg_wm_base::XdgWmBase;
		let base_surface = self.base_surface as *mut wl_surface::WlSurface;

		let xdg_surface = unsafe { (*wm_base).get_xdg_surface(&*base_surface, qh, ()) };
		let toplevel = xdg_surface.get_toplevel(qh, ());
		toplevel.set_title("A fantastic window!".into());

		unsafe { (*base_surface).commit() };

		self.xdg_surface =
			&mut WaylandFrame { xdg_surface, toplevel } as *mut WaylandFrame as *const c_void;
	}
}

pub fn supports_blur() -> bool
{
	true
}

/// Detect the current DE/WM that the program is beeing executed
fn get_de() -> DE
{
	let desktop = env::var("XDG_CURRENT_DESKTOP")
		.unwrap_or_else(|_| {
			warn!("missing XDG_CURRENT_DESKTOP");
			String::from("")
		});

	if desktop == "" { return DE::Unknown }
	if desktop.contains("KDE") { return DE::Kde }
	if desktop.contains("Hyprland") { return DE::Hyprland }

	DE::Other
}

struct WaylandFrame {
	xdg_surface: xdg_surface::XdgSurface,
	toplevel: xdg_toplevel::XdgToplevel,
}

pub trait WaylandDecoration
{
	/// Creates a native window frame decoration for Linux DE/WM
	fn new(title: &str, _width: f64, _height: f64) -> Decoration;
	fn make_view();
	fn apply_blur(&self);
	fn init_event_state() -> wayland_client::EventQueue<State>;
}

// wayland_protocols (which include wayland_client) failed to build documentation on version 0.31.12 thks!!
impl WaylandDecoration for Decoration
{
	fn new(_title: &str, _width: f64, _height: f64) -> Decoration
	{
		let _state = todo!();//State::init_xdg_surface()
		let frame = todo!();

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
		return Decoration {
			mode: DecorationMode::ServerSide,
			frame,
			backend: SurfaceBackend::Linux {}
		};
	}

	fn init_event_state() -> EventQueue<State>
	{
		let connection = Connection::connect_to_env().unwrap();
		let event_queue: EventQueue<State> = connection.new_event_queue();

		event_queue
	}

	fn make_view() {}

	fn apply_blur(&self)
	{
		if !supports_blur() {
			warn!("couldn't set window blur, desktop doesn't implement protocol");
			return
		}

		/**
		 * the `hyprland_surface_manager_v1` protocol already covers this, skip
		 * <https://wayland.app/protocols/hyprland-surface-v1>
		 */
		if get_de() == DE::Hyprland
			{ return }
	}
}

// https://github.com/Smithay/wayland-rs/blob/master/wayland-client/examples/simple_window.rs
impl Dispatch<wl_registry::WlRegistry, ()> for State
{
	fn event(
		state: &mut Self,
		registry: &wl_registry::WlRegistry,
		event: wl_registry::Event,
		_: &(),
		_: &Connection,
		qh: &QueueHandle<Self>,
	) {
		if let wl_registry::Event::Global { name, interface, .. } = event {
			match &interface[..] {
				"wl_compositor" => {
					let compositor =
						registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
					let mut surface = compositor.create_surface(qh, ());
					state.base_surface = &mut surface as *mut wl_surface::WlSurface as *mut c_void;

					//if state.wm_base.is_some() && state.xdg_surface.is_none() {
					state.init_xdg_surface(qh);
				}
				"wl_seat" => {
					registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
				}
				"xdg_wm_base" => {
					let mut wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, 1, qh, ());
					state.wm_base =
						&mut wm_base as *mut xdg_wm_base::XdgWmBase as *mut c_void;

					//let base_surface = state.base_surface as *mut wl_surface::WlSurface;
					//let xdg_surface = state.xdg_surface as *mut wl_surface::WlSurface;

					// I cant test if is Some or None here
					// for now I will just pretend that xdg_surface doesn't exist
					//if (*base_surface).is_alive() && (*xdg_surface).is_not_alive()
					state.init_xdg_surface(qh);
				}
				_ => {}
			}
		}
	}
}

delegate_noop!(State: ignore wl_compositor::WlCompositor);
delegate_noop!(State: ignore wl_surface::WlSurface);
delegate_noop!(State: ignore wl_shm::WlShm);
delegate_noop!(State: ignore wl_shm_pool::WlShmPool);
delegate_noop!(State: ignore wl_buffer::WlBuffer);

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for State {
	fn event(
		_: &mut Self,
		wm_base: &xdg_wm_base::XdgWmBase,
		event: xdg_wm_base::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		if let xdg_wm_base::Event::Ping { serial } = event {
			wm_base.pong(serial);
		}
	}
}

impl Dispatch<xdg_surface::XdgSurface, ()> for State {
	fn event(
		state: &mut Self,
		xdg_surface: &xdg_surface::XdgSurface,
		event: xdg_surface::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		if let xdg_surface::Event::Configure { serial, .. } = event
		{
			xdg_surface.ack_configure(serial);
			state.configured = true;
			let surface = state.base_surface as *mut wl_surface::WlSurface;
			let buffer = state.buffer as *mut wl_buffer::WlBuffer;

			//if let Some(ref buffer) = state.buffer
			unsafe { (*surface).attach(Some(&*buffer), 0, 0) };
			unsafe { (*surface).commit() };
		}
	}
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for State {
	fn event(
		state: &mut Self,
		_: &xdg_toplevel::XdgToplevel,
		event: xdg_toplevel::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		if let xdg_toplevel::Event::Close = event {
			state.running = false;
		}
	}
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
	fn event(
		_: &mut Self,
		seat: &wl_seat::WlSeat,
		event: wl_seat::Event,
		_: &(),
		_: &Connection,
		qh: &QueueHandle<Self>,
	) {
		if let wl_seat::Event::Capabilities { capabilities: WEnum::Value(capabilities) } = event
		{
			if capabilities.contains(wl_seat::Capability::Keyboard) {
				seat.get_keyboard(qh, ());
			}
		}
	}
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
	fn event(
		state: &mut Self,
		_: &wl_keyboard::WlKeyboard,
		event: wl_keyboard::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		if let wl_keyboard::Event::Key { key, .. } = event
		{
			if key == 1 /*ESC */ {
				state.running = false;
			}
		}
	}
}
