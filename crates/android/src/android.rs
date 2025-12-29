#[cfg(target_os = "android")]
use ndk_sys::ANativeActivity;
use std::ffi::c_void;
use log::debug;

/// Android program init funcion
/// source example <https://docs.rs/ndk-sys/latest/ndk_sys/fn.ANativeActivity_onCreate.html>
#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn ANativeActivity_onCreate(
	activity: *mut ANativeActivity,
	saved_state: *mut c_void,
	saved_state_size: usize,
) {
	debug!("android");
	unsafe {
		let vm = (*activity).vm;
		let asset_manager = (*activity).assetManager;
	}
}
