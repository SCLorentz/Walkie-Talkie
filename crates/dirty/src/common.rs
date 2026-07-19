#![no_std]
#![feature(rustc_private)]
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
//! That's why there are so many `#[deny]` configs (clippy helps a lot here)
#![doc = include_str!("../README.md")]

extern crate alloc;
pub use alloc::{
	boxed::Box,
	format, slice, str,
	string::{String, ToString},
	vec::Vec,
};

use libc::{
    pthread_create, pthread_join, pthread_t, pthread_kill, pthread_self,
    c_void,
};

/// just a void type
#[repr(C)]
#[allow(non_camel_case_types)]
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
    pub fn to_handle<T>(val: T) -> *mut void {
        Box::into_raw(Box::new(val)).cast::<void>()
    }

    /// Espects a T return type and a Boxed `void` pointer to get value inside the Box
    #[must_use]
    #[inline]
    pub fn from_handle<T>(ptr: *const void) -> T {
        unsafe { *Box::from_raw(ptr as *mut T) }
    }
}

/// int32 bool type
pub static TRUE: u32 = 1;
/// int32 bool type
pub static FALSE: u32 = 0;

/// Type for a function repr in C that takes `void* arg` and returns `void*`
#[cfg(target_family = "unix")]
pub type AnyFunction = extern "C" fn(*mut c_void) -> *mut c_void;

/// This is a thread interface with the C implementation
#[derive(Debug)]
#[cfg(target_family = "unix")]
pub struct Thread {
    /// The function beeing executed in the new thread
    pub thread_main: AnyFunction,
    /// If the thread is active, this contains the ID and `pthread_t` struct
    #[allow(unused)]
    thread: pthread_t,
}

#[cfg(target_family = "unix")]
impl Thread {
    /// Creates a new thread with the field `thread_id` and a provided function
    #[must_use]
    pub fn create(thread_main: AnyFunction) -> Self
    {
        let mut thread: pthread_t = unsafe { core::mem::zeroed() };

        let ret = unsafe { pthread_create(
            &mut thread,
            core::ptr::null(),
            thread_main,
            core::ptr::null_mut(),
        )};

        assert_eq!(ret, 0);

        let _ = unsafe { pthread_join(thread, core::ptr::null_mut()) };
        Self {
            thread_main,
            thread,
        }
    }

    /**
     * Returns the ID for the active thread
     * # Errors
     * this will return Err(UnexpectedError) if the thread does not exist
     */
	pub fn get_id(&mut self) -> Result<i32, WResponse> {
		let Ok(val) = (unsafe { pthread_self().try_into() })
			else { return Err(WResponse::UnexpectedError) };
		Ok(val)
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
		if unsafe { pthread_kill(self.thread, 0) } == -1
			{ return Err(WResponse::InvalidRequest) }
		Ok(())
	}
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
pub enum WResponse {
    /// The binary does not support this function
    BinarySpecificLimitation = 500,
    /// Tried to use a wayland protocol that wasn't implemented on the compositor
    ProtocolNotSuported = 501,
    /// tried to access something and the request was denied by the OS
    AccessDenied = 502,
    /// Recived a value that wasn't supposed to be empty or an error
    UnexpectedError = 503,
    /// this will cause a buffer overflow
    OutOfBounds = 504,
    /// user tried to do an impossible action
    InvalidRequest = 505,
    /// Failed to do the requested action
    Faliure,
    /// Tried to do something with the window, but the compositor denied
    ForbiddenByCompositor = 601,
    /// Something for macos
    ChannelInUse = 400,
    /// A dynamic linked dependency was missing on execution
    MissingDependencies = 401,
}

// for some reason I can't move this to app.rs
/// Abtraction layer for multiple OS support
#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceWrapper(pub *mut void);

impl SurfaceWrapper {
    /// Create a new wrapper
    #[must_use]
    pub fn new<T>(wrap: T) -> Self {
        SurfaceWrapper(void::to_handle(wrap))
    }
    /// Is wrapper valid?
    #[must_use]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    /// cast wrapper to original value
    #[must_use]
    pub fn cast<T>(&self) -> T {
        void::from_handle(self.0)
    }
}

/// RGB color implementation
/// reference: <https://github.com/seancroach/hex_color/blob/main/src/lib.rs>
#[derive(PartialEq, Clone, Debug)]
#[allow(missing_docs, non_snake_case)]
pub struct Color {
    pub R: u8,
    pub G: u8,
    pub B: u8,
    pub A: u8,
}

// 0.0039215686274 <- I got here before realizing that this is just 1/255
/// Just a simple constant to normalize the RGB (0-255) value to the normal shader value (0-1)
/// In old days people used to this trick: `(x * 257) >> 16` (nice ✧ദ്ദി)
const RGB_NORM: f64 = 1.0 / 255.0;

impl Color {
    /// Create new color value
    #[must_use]
    #[allow(non_snake_case)]
    pub fn from(R: u8, G: u8, B: u8, A: u8) -> Self {
        Self { R, G, B, A }
    }

    /// Converts this to a functional method to be used inside functions
    #[must_use]
    pub fn to_default(&self) -> (f64, f64, f64, f64) {
        (
            f64::from(self.R) * RGB_NORM,
            f64::from(self.G) * RGB_NORM,
            f64::from(self.B) * RGB_NORM,
            f64::from(self.A) * RGB_NORM,
        )
    }
}
