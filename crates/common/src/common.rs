#![no_std]
#![no_main]

/// This is a helper crate, with minimum dependencies, not even std included
/// Things in here should and will be dirty

use core::ffi::c_void;
extern crate alloc;
use alloc::boxed::Box;

/// This is an abstraction layer used to get the surface easly
/// ```rust
/// impl TSurfaceBackend for Wrapper {
/// 	fn get_surface(backend: *mut c_void) -> *mut c_void
/// 	{
/// 		let backend: Self = unsafe { from_handle(backend) };
///			return backend.ns_view;
/// 	}
/// }
/// ```
pub trait SurfaceBackend {
	fn get_surface(backend: *mut c_void) -> *mut c_void;
}

#[inline]
pub fn to_handle<T>(val: T) -> *mut c_void
	{ Box::into_raw(Box::new(val)) as *mut c_void }

#[inline]
pub unsafe fn from_handle<T>(ptr: *const c_void) -> T
{
	let value = unsafe { Box::from_raw(ptr as *mut T) };
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

// https://github.com/seancroach/hex_color/blob/main/src/lib.rs
#[derive(PartialEq, Clone, Debug)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub fn from(r: u8, g: u8, b: u8, a: u8) -> Self
	{
		Self { r, g, b, a }
	}
}
