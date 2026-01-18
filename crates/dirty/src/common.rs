#![no_std]
#![allow(non_snake_case, non_camel_case_types, clippy::tabs_in_doc_comments)]
#![deny(
	deprecated,
	rust_2018_idioms,
	clippy::shadow_unrelated,
	unreachable_code,
	unused_imports,
	unused_variables,
	warnings,
	clippy::all,
	clippy::pedantic,
	unsafe_op_in_unsafe_fn,
	clippy::unwrap_used,
	clippy::expect_used,
	clippy::panic,
	clippy::todo,
	clippy::unimplemented,
	clippy::shadow_reuse,
	clippy::shadow_same,
)]
//! This is a helper crate, with minimum dependencies, not even std included
//!
//! Things in here should and will be dirty!
#![doc = include_str!("../README.md")]

extern crate alloc;
pub use alloc::boxed::Box;
pub mod syscall;

#[repr(C)]
pub struct SocketResponse
{
	pub status: i32,
	pub server_socket: i32,
}

mod socket {
	use crate::{SocketResponse, void};

	unsafe extern "C" {
		pub(crate) fn create_socket(address: *mut void) -> SocketResponse;
		pub(crate) fn read_socket(server_socket: i32, ch: *mut void) -> *mut void;
		pub(crate) fn write_socket(server_socket: i32, ch: *mut void);
		pub(crate) fn close_socket(server_socket: i32);
	}
}

pub struct Socket {
	socket_id: Option<i32>,
}

impl Socket {
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

	#[must_use]
	pub fn read_socket(&self, ch: &'static [u8]) -> Option<Box<&[f8]>>
	{
		let socket_id = self.socket_id?;
		let response = unsafe { socket::read_socket(socket_id, void::to_handle(ch)) };
		Some(Box::new(void::from_handle(response)))
	}

	pub fn write_socket(&self, ch: &'static [u8])
	{
		let Some(socket_id) = self.socket_id else { return };
		unsafe { socket::write_socket(socket_id, void::to_handle(ch)) };
	}

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
pub type f8 = i8;

// fuck the ABI
/// Always trust the f8 type. The ABI is not your friend!
///
/// This can be ether i8 or u8 depending on the current ABI specification used
#[cfg(all(target_os = "linux", target_env = "musl", target_arch = "aarch64"))]
pub type f8 = u8;

#[repr(C)]
pub struct void {
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

pub enum WRequestResult<T> {
	Fail(WResponse),
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
	#[must_use]
	pub fn new<T>(wrap: T) -> Self { SurfaceWrapper(void::to_handle(wrap)) }
	#[must_use]
	pub fn is_null(&self) -> bool { self.0.is_null() }
	#[must_use]
	pub fn cast<T>(&self) -> T { void::from_handle(self.0) }
}

// https://github.com/seancroach/hex_color/blob/main/src/lib.rs
#[derive(PartialEq, Clone, Debug)]
pub struct Color { pub R: u8, pub G: u8, pub B: u8, pub A: u8, }

impl Color {
	#[must_use]
	pub fn from(R: u8, G: u8, B: u8, A: u8) -> Self { Self { R, G, B, A } }
}

#[derive(Clone, PartialEq, Debug)]
pub struct String {
	vec: *mut void
}

impl String
{
	#[must_use]
	pub fn from(val: &str) -> Self
		{ Self { vec: void::to_handle(val) } }

	pub fn as_str(&mut self) -> &str
		{ void::from_handle::<&str>(self.vec) }
}
