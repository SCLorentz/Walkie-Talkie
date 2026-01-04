#![no_std]
#![no_main]

/// This is a helper crate, with minimum dependencies, not even std included
/// Things in here should and will be dirty

use core::ffi::c_void;
extern crate alloc;
use alloc::boxed::Box;

#[derive(PartialEq, Debug, Clone)]
pub struct MacWrapper {
	pub ns_view: *mut c_void,		// NSView
	pub rect: *const c_void,		// NSRect
	pub app: *const c_void,			// NSApplication
}

#[derive(Clone, PartialEq, Debug)]
pub enum SurfaceBackend {
	MacOS(MacWrapper),
	Windows {},
	/// <https://github.com/Smithay/wayland-rs/blob/master/wayland-client/examples/simple_window.rs>
	Linux {
		state: *mut c_void
	},
	Headless,
}

pub fn to_handle<T>(val: T) -> *mut c_void
	{ Box::into_raw(Box::new(val)) as *mut c_void }

pub unsafe fn from_handle<T>(ptr: *const c_void) -> T
{
	let value = Box::from_raw(ptr as *mut T);
	*value
}

pub enum WRequestResult<T> {
	Fail(WResponse),
	Success(T)
}

/** Errors
 * 6## : Window Request Failed
 * 4## : Rendererer Request Failed
 * 5## : General Program limitation
 */

#[derive(Debug)]
pub enum WResponse
{
	BinarySpecificLimitation	= 500,
	ProtocolNotSuported			= 501,
	AccessDenied				= 502,
	NotSupported				= 600,
	NotImplementedInCompositor	= 601,
	ChannelInUse				= 400,
	MissingDependencies			= 401,
}


#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceWrapper(pub *mut c_void);

impl SurfaceWrapper
{
	pub fn new<T>(wrap: T) -> Self
		{ SurfaceWrapper(to_handle(wrap)) }

	pub fn is_null(&self) -> bool
		{ self.0.is_null() }

	pub unsafe fn cast<T>(&self) -> T
		{ unsafe { from_handle(self.0) } }
}
