#![no_std]
#![feature(core_intrinsics, stmt_expr_attributes)]
#![deny(
	deprecated,
	rust_2018_idioms,
	unreachable_code,
	unused_imports,
	unused_variables,
	unsafe_op_in_unsafe_fn,
	missing_docs,
	warnings,
	clippy::all,
	clippy::shadow_unrelated,
	clippy::pedantic,
	clippy::unwrap_used,
	clippy::expect_used,
	clippy::panic,
	clippy::todo,
	clippy::unimplemented,
	clippy::shadow_reuse,
	clippy::shadow_same,
	clippy::dbg_macro,
	clippy::print_stdout,
	clippy::print_stderr,
	clippy::indexing_slicing,
	clippy::arithmetic_side_effects,
	clippy::float_arithmetic,
	clippy::unwrap_in_result,
	clippy::exit,
	clippy::wildcard_imports,
	clippy::missing_docs_in_private_items,
	clippy::doc_markdown,
	clippy::empty_docs,
	clippy::unwrap_or_default,
	clippy::match_wild_err_arm,
	clippy::needless_pass_by_value,
	clippy::redundant_closure,
	clippy::large_stack_arrays,
)]
#![allow(clippy::tabs_in_doc_comments, internal_features)]
//! This is a helper crate, with minimum dependencies, not even std included
//!
//! Things in here should and will be dirty!
//! That's why there are so many `#[deny]` configs (clippy helps a lot here)
#![doc = include_str!("../README.md")]

extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::string::{String, ToString};
use core::{slice, str};

/// OS specific methods based on systemcalls (ASM)
pub mod syscall;

/// This represents the possible state of the socket response
#[cfg(not(target_os = "windows"))]
#[repr(C)]
pub struct SocketResponse
{
	/**
	 * This represents the state of the connection.
	 *
	 * If `-1`, then the connection failed
	 */
	pub status: i32,
	/**
	 * if the connection was sucesseful,
	 * then this will return the int id of the server socket
	 */
	pub server_socket: i32,
}

#[cfg(not(target_os = "windows"))]
/// This will handle with our C imports from `unix/socket.c`
mod socket {
	use crate::{SocketResponse, void};

	unsafe extern "C" {
		pub(crate) fn create_socket(address: *mut void) -> SocketResponse;
		pub(crate) fn read_socket(server_socket: i32, ch: *mut void) -> *mut void;
		pub(crate) fn write_socket(server_socket: i32, ch: *mut void);
		pub(crate) fn close_socket(server_socket: i32);
		pub(crate) fn getenv(find: *const i8) -> *const i8;
	}
}

/**
 * The function will return the size of a string.
 *
 * This works by prompting the first byte of the string (represented by the pointer `p`),
 * then the loop will get all following characters until the `\0` (string termination character)
 *
 * this *may cause "SIGSEGV (Address boundary error)" ¯\(ツ)/¯
 *
 * the code is bad...
 */
fn c_strlen(mut p: *const u8, bypass_hardcoded_limit: Option<usize>) -> usize
{
	// to avoid (but not prevent) boundary related errors,
	// 1024 is a hardcoded limit in case `\0` doesn't exist
	let limit = bypass_hardcoded_limit.unwrap_or(1024);
	unsafe {
		let mut len = 0;
		while len < limit && *p != 0 {
			#[allow(clippy::arithmetic_side_effects)]
			len += 1;		// save how many iterations the loop had
			p = p.add(1);	// increments the position of the pointer by one
		}
		len
	}
}

/**
 * this basicly creates a new string based on the initial location and the final size
 * `ptr` is the first byte and `len` is how long it should read the string
 */
fn getenv_str(ptr: *const u8) -> &'static [u8]
{
	unsafe {
		let len = c_strlen(ptr, None);
		slice::from_raw_parts(ptr, len)
	}
}

/**
 * this converts the raw byte value into a readable string
 * <https://doc.rust-lang.org/stable/core/str/fn.from_utf8.html>
 */
fn convert_bytes_to_string(bytes: &[u8]) -> Option<String>
{
	let Ok(str) = str::from_utf8(bytes) else { return None };
	Some(ToString::to_string(str))
}


/// this will check the environ array and search for an specific keyword
///
/// example:
/// ```rust
///	let path = getenv(b"PATH");
/// ```
// all of this just makes me hate the `i8 != u8` thing. Fuck the ABI
#[cfg(not(target_os = "windows"))]
#[must_use]
pub fn getenv(find: &'static str) -> Option<String>
{
	let raw_pointer = unsafe { socket::getenv(find.as_ptr().cast::<i8>()) };
	let string = getenv_str(raw_pointer.cast::<u8>());
	convert_bytes_to_string(string)
}

#[cfg(not(target_os = "windows"))]
/// The default Socket struct.
pub struct Socket {
	/// The same field of `SocketResponse.server_socket`.
	/// This time in the rust layout.
	/// Can be None in case the `socket::create_socket()` returned `-1` (or err in the c lib for sockets)
	socket_id: Option<i32>,
}

#[cfg(not(target_os = "windows"))]
impl Socket {
	/// Create a new socket connection to the defined address
	#[must_use]
	pub fn new(address: &'static [u8]) -> Self
	{
		let response: SocketResponse =
			unsafe { socket::create_socket(void::to_handle(address)) };

		if response.status == -1 {
			return Socket { socket_id: None };
		}

		let socket_id = Some(response.server_socket);
		Socket { socket_id, }
	}

	/// read the socket signal
	#[must_use]
	pub fn read_socket(&self, ch: &'static [u8]) -> Option<Box<&[f8]>>
	{
		let socket_id = self.socket_id?;
		let response = unsafe { socket::read_socket(socket_id, void::to_handle(ch)) };
		Some(Box::new(void::from_handle(response)))
	}

	/// write a socket signal
	pub fn write_socket(&self, ch: &'static [u8])
	{
		let Some(socket_id) = self.socket_id else { return };
		unsafe { socket::write_socket(socket_id, void::to_handle(ch)) };
	}

	/// close the connection with the socket
	pub fn close_socket(&self)
	{
		let Some(socket_id) = self.socket_id else { return };
		unsafe { socket::close_socket(socket_id) }
	}
}

/// Always trust the f8 type. The ABI is not your friend!
///
/// This can be ether i8 or u8 depending on the current ABI specification used
#[cfg(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")))]
#[allow(non_camel_case_types)]
pub type f8 = i8;

// fuck the ABI
/// Always trust the f8 type. The ABI is not your friend!
///
/// This can be ether i8 or u8 depending on the current ABI specification used
#[cfg(all(target_os = "linux", target_env = "musl", target_arch = "aarch64"))]
#[allow(non_camel_case_types)]
pub type f8 = u8;

/// just a void type
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct void {
	/// This is a pointer of nothing
	/// An u8 array of size 0
	/// similar as how `core::ffi::c_void` works
	_private: [u8; 0],
}

impl void {
	/// Get a T type value and stores it safely as a generic type
	#[must_use]
	#[inline]
	pub fn to_handle<T>(val: T) -> *mut void
		{ Box::into_raw(Box::new(val)).cast::<void>() }

	/// Espects a T return type and a Boxed `void` pointer to get value inside the Box
	#[must_use]
	#[inline]
	pub fn from_handle<T>(ptr: *const void) -> T
		{ unsafe { *Box::from_raw(ptr as *mut T) } }
}

/// int32 bool type
pub static TRUE: u32 = 1;
/// int32 bool type
pub static FALSE: u32 = 0;

/// Handle with errors with this type
pub enum WRequestResult<T> {
	/// Function failed
	Fail(WResponse),
	/// Function succeded
	Success(T)
}

/** Possible responses
 *
 * 6## : Window Request Failed
 *
 * 4## : Rendererer Request Failed
 *
 * 5## : General Program limitation
 */

#[derive(Debug)]
pub enum WResponse
{
	/// The binary does not support this function
	BinarySpecificLimitation	= 500,
	/// Tried to use a wayland protocol that wasn't implemented on the compositor
	ProtocolNotSuported			= 501,
	/// tried to access something and the request was denied by the OS
	AccessDenied				= 502,
	/// Recived a value that wasn't supposed to be empty or an error
	UnexpectedError				= 503,
	/// Tried to do something with the window, but the compositor denied
	ForbiddenByCompositor		= 601,
	/// Something for macos
	ChannelInUse				= 400,
	/// A dynamic linked dependency was missing on execution
	MissingDependencies			= 401,
}

/// Abtraction layer for multiple OS support
#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceWrapper(pub *mut void);

impl SurfaceWrapper
{
	/// Create a new wrapper
	#[must_use]
	pub fn new<T>(wrap: T) -> Self { SurfaceWrapper(void::to_handle(wrap)) }
	/// Is wrapper valid?
	#[must_use]
	pub fn is_null(&self) -> bool { self.0.is_null() }
	/// cast wrapper to original value
	#[must_use]
	pub fn cast<T>(&self) -> T { void::from_handle(self.0) }
}

/// RGB color implementation
/// reference: <https://github.com/seancroach/hex_color/blob/main/src/lib.rs>
#[derive(PartialEq, Clone, Debug)]
#[allow(missing_docs, non_snake_case)]
pub struct Color { pub R: u8, pub G: u8, pub B: u8, pub A: u8, }

impl Color {
	/// Create new color value
	#[must_use]
	#[allow(non_snake_case)]
	pub fn from(R: u8, G: u8, B: u8, A: u8) -> Self { Self { R, G, B, A } }
}
