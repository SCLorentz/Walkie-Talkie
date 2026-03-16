#![no_std]
//#![feature(core_intrinsics, stmt_expr_attributes)]
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
	missing_debug_implementations,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates,
	unused_import_braces,
	unused_qualifications,
	unused_results,
	macro_use_extern_crate
)]
#![allow(clippy::tabs_in_doc_comments, internal_features)]
//! This is a helper crate, with minimum dependencies, not even std included
//!
//! Things in here should and will be dirty!
#![cfg_attr(doc, doc = include_str!("../README.md"))]

extern crate alloc;
pub use alloc::{
	boxed::Box,
	string::{String, ToString},
	slice,
	str,
	vec::Vec,
	format,
};

/// OS specific methods based on systemcalls (ASM)
pub mod syscall;

/// This represents the possible state of the socket response
#[cfg(target_family = "unix")]
#[repr(C)]
#[derive(Debug)]
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

/// Wrapper type for the C `struct` that stores threads
#[cfg(target_family = "unix")]
#[derive(Debug)]
#[repr(C)]
#[allow(non_camel_case_types, clippy::missing_docs_in_private_items, reason = "this is a struct wrapper for the C mod")]
struct c_Thread
{
	pub id: i32,
	pub thread: *mut void,
}

#[cfg(target_family = "unix")]
/// This will handle with our C imports from `unix/socket.c`
mod unix {
	use crate::{c_Thread, AnyFunction};

	unsafe extern "C" {
		pub(crate) fn create_thread(function: AnyFunction) -> c_Thread;
		pub(crate) fn kill_thread(thread: &c_Thread);
	}
}

/// Type for a function repr in C that takes `void* arg` and returns `void*`
#[cfg(target_family = "unix")]
pub type AnyFunction = extern "C" fn(*mut void) -> *mut void;

/// This is a thread interface with the C implementation
#[derive(Debug)]
#[cfg(target_family = "unix")]
pub struct Thread {
	/// The function beeing executed in the new thread
	pub function: AnyFunction,
	/// If the thread is active, this contains the ID and `pthread_t` struct
	thread: Option<c_Thread>,
}

#[cfg(target_family = "unix")]
impl Thread
{
	/// Creates a new thread with the field `thread_id` and a provided function
	#[must_use]
	pub fn default(function: AnyFunction) -> Self
	{
		Self {
			function,
			thread: None
		}
	}

	/// Runs the `&self.thread`
	pub fn run(&mut self)
	{
		let thread = unsafe { unix::create_thread(self.function) };
		self.thread = Some(thread);
	}

	/**
	 * Kills the specified running thread
	 *
	 * # Errors
	 *
	 * if the user tries to kill a thread that is not running, it will return Err(InvalidRequest)
	 */
	pub fn kill(&self) -> Result<(), WResponse>
	{
		let Some(ref thread) = self.thread else { return Err(WResponse::InvalidRequest) };
		unsafe { unix::kill_thread(thread); }
		Ok(())
	}
}

// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
/**
 * this transforms any generic struct type variable into raw data
**/
pub unsafe fn as_u8_slice<T: Sized, const N: usize>(mut p: T) -> [u8; N]
{
	#[allow(trivial_casts)]
	let ptr = &mut p as *const T;
	let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8,
		size_of::<T>()) };
	let mut ret = [0u8; N];
	ret[..slice.len()].copy_from_slice(slice);
	ret
}

/*#[cfg(target_family = "unix")]
#[derive(Debug, Clone, PartialEq)]
/// The default Socket struct.
pub struct Socket {
	/// The same field of `SocketResponse.server_socket`.
	/// This time in the rust layout.
	/// Can be None in case the `socket::create_socket()` returned `-1` (or err in the c lib for sockets)
	socket_id: Option<i32>,
}

#[cfg(target_family = "unix")]
impl Socket {
	/// Create a new socket connection to the defined address
	#[inline]
	#[must_use]
	pub fn connect(address: &str) -> Self
	{
		debug!("connecting to socket: {:?}", address);
		let response: SocketResponse =
			unsafe { unix::create_socket(void::to_handle(address)) };

		if response.status == -1 {
			return Self { socket_id: None };
		}

		let socket_id = Some(response.server_socket);
		Self { socket_id, }
	}

	/// read the socket signal
	#[inline]
	#[must_use]
	pub fn recv(&self) -> Option<[u8; 4096]>
	{
		let mut buf = [0u8; 4096];
		let socket_id = self.socket_id?;
		debug!("socket id: {}", socket_id);
		unsafe { unix::recv(socket_id, &mut buf, 4096, 0x40) };
		debug!("buf: {:?}", buf);

		Some(buf)
	}

	/// write a socket signal
	pub fn send(&self, ch: [u8; 4096])
	{
		let Some(socket_id) = self.socket_id else { return };
		unsafe { unix::send(socket_id, ch, 4096, 0x40) };
	}

	/// close the connection with the socket
	pub fn close(&self)
	{
		let Some(socket_id) = self.socket_id else { return };
		unsafe { unix::close_socket(socket_id) }
	}
}*/

/// Always trust the f8 type. The ABI is not your friend!
///
/// This can be ether i8 or u8 depending on the current ABI specification used
#[cfg(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")))]
#[expect(non_camel_case_types, reason = "this should use the same format. as i8/u8")]
pub type f8 = i8;

// fuck the ABI
/// Always trust the f8 type. The ABI is not your friend!
///
/// This can be ether i8 or u8 depending on the current ABI specification used
#[cfg(all(target_os = "linux", target_env = "musl", target_arch = "aarch64"))]
#[expect(non_camel_case_types)]
pub type f8 = u8;

/// just a void type
#[repr(C)]
#[derive(Debug)]
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
	pub fn to_handle<T>(val: T) -> *mut Self
		{ Box::into_raw(Box::new(val)).cast::<Self>() }

	/// Espects a T return type and a Boxed `void` pointer to get value inside the Box
	#[must_use]
	#[inline]
	// cast::<T> doesn't work here
	pub fn from_handle<T>(ptr: *const Self) -> T
		{ unsafe { *Box::from_raw(ptr as *mut T) } }
}

/// int32 bool type
pub static TRUE: u32 = 1;
/// int32 bool type
pub static FALSE: u32 = 0;

/** Possible responses
 *
 * 6## : Window Request Failed
 *
 * 4## : Rendererer Request Failed
 *
 * 5## : General Program limitation
 */
#[derive(Debug)]
#[non_exhaustive]
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
	/// this will cause a buffer overflow
	OutOfBounds					= 504,
	/// user tried to do an impossible action
	InvalidRequest				= 505,
	/// Tried to do something with the window, but the compositor denied
	ForbiddenByCompositor		= 601,
	/// The code expected to be run on the main thread (mostly a problem on macOS)
	MainThreadError				= 602,
	/// Something for macos
	ChannelInUse				= 400,
	/// A dynamic linked dependency was missing on execution
	MissingDependencies			= 401,
}

/// Abtraction layer for multiple OS support
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub struct SurfaceWrapper(pub *mut void);

impl SurfaceWrapper
{
	/// Create a new wrapper
	#[inline]
	#[must_use]
	pub fn new<T>(wrap: T) -> Self { Self(void::to_handle(wrap)) }
	/// Is wrapper valid?
	#[inline]
	#[must_use]
	pub const fn is_null(&self) -> bool { self.0.is_null() }
	/// cast wrapper to original value
	#[inline]
	#[must_use]
	pub fn cast<T>(&self) -> T { void::from_handle(self.0) }
}

/// RGB color implementation
/// reference: <https://github.com/seancroach/hex_color/blob/main/src/lib.rs>
#[derive(PartialEq, Eq, Clone, Debug)]
#[non_exhaustive]
#[expect(missing_docs, non_snake_case, clippy::min_ident_chars, reason = "using the default nom. for RGBA")]
pub struct Color { pub R: u8, pub G: u8, pub B: u8, pub A: u8, }

// 0.0039215686274 <- I got here before realizing that this is just 1/255
/// Just a simple constant to normalize the RGB (0-255) value to the normal shader value (0-1)
/// In old days people used to this trick: `(x * 257) >> 16` (nice ✧ദ്ദി).
const RGB_NORM: f64 = 1.0 / 255.0;

impl Color {
	/// Create new color value.
	#[inline]
	#[must_use]
	#[expect(non_snake_case, clippy::min_ident_chars, reason = "using the default nom. for RGBA")]
	pub const fn from(R: u8, G: u8, B: u8, A: u8) -> Self { Self { R, G, B, A } }

	/// Converts this to a functional method to be used inside functions.
	#[inline]
	#[must_use]
	#[expect(clippy::float_arithmetic, reason = "necessary for the value normalization for GPU")]
	pub fn to_default(&self) -> ( f64, f64, f64, f64 )
	{(
		f64::from(self.R) * RGB_NORM,
		f64::from(self.G) * RGB_NORM,
		f64::from(self.B) * RGB_NORM,
		f64::from(self.A) * RGB_NORM,
	)}
}
