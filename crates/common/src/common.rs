#![no_std]
#![no_main]
#![allow(non_snake_case)]

/// This is a helper crate, with minimum dependencies, not even std included
/// Things in here should and will be dirty

extern crate alloc;
pub use alloc::boxed::Box;

#[repr(C)]
pub struct void {
	_private: [u8; 0],
}

/// int32 bool type
pub static TRUE: u32 = 1;
/// int32 bool type
pub static FALSE: u32 = 0;

/// Get a T type value and stores it safely as a generic type
#[inline]
pub fn to_handle<T>(val: T) -> *mut void
	{ Box::into_raw(Box::new(val)) as *mut void }

/// Espects a T return type and a Boxed c_void pointer to get value inside the Box
#[inline]
pub fn from_handle<T>(ptr: *const void) -> T
	{ unsafe { *Box::from_raw(ptr as *mut T) } }

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
pub struct SurfaceWrapper(pub *mut void);

impl SurfaceWrapper
{
	pub fn new<T>(wrap: T) -> Self { SurfaceWrapper(to_handle(wrap)) }
	pub fn is_null(&self) -> bool { self.0.is_null() }
	pub unsafe fn cast<T>(&self) -> T {  from_handle(self.0) }
}

// https://github.com/seancroach/hex_color/blob/main/src/lib.rs
#[derive(PartialEq, Clone, Debug)]
pub struct Color { pub R: u8, pub G: u8, pub B: u8, pub A: u8, }

impl Color {
	pub fn from(R: u8, G: u8, B: u8, A: u8) -> Self { Self { R, G, B, A } }
}

// https://github.com/tokio-rs/mio
// https://www.zupzup.org/epoll-with-rust/index.html
// non-blocking I/O
/*#[allow(unused_macros)]
macro_rules! syscall {
	($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
		let res = unsafe { libc::$fn($($arg, )*) };
		if res == -1 {
			Err(std::io::Error::last_os_error())
		} else {
			Ok(res)
		}
	}};
}*/
