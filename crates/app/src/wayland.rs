use wayland_client::{
	Display, GlobalManager,
};

use wayland_protocols::xdg::decoration::zv1::client::{
	zxdg_decoration_manager_v1::ZxdgDecorationManagerV1
};

pub fn detect_decorations(display: &Display) -> bool
{
	let globals = GlobalManager::new(display);

	display.flush().unwrap();
	display.dispatch_pending(&mut ()).unwrap();

	globals
		.instantiate_exact::<ZxdgDecorationManagerV1>(1)
		.is_ok()
}

//use wayland_protocols::unstable::xdg_decoration::v1::client::zxdg_toplevel_decoration_v1::Mode;

#[allow(unused)]
pub fn detect_decorations() -> bool
{
	true
}
