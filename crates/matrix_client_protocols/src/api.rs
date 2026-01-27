#![no_std]
#![deny(
	deprecated,
	rust_2018_idioms,
	clippy::shadow_unrelated,
	unreachable_code,
	unused_imports,
	unused_variables,
	unused_extern_crates,
	unused_import_braces,
	unused_qualifications,
	unused_results,
	unsafe_op_in_unsafe_fn,
	clippy::unwrap_used,
	clippy::expect_used,
	clippy::shadow_reuse,
	clippy::shadow_same,
	clippy::dbg_macro,
	clippy::print_stdout,
	clippy::print_stderr,
	clippy::panic,
	clippy::indexing_slicing,
	clippy::float_arithmetic,
	clippy::unwrap_in_result,
	clippy::exit,
	clippy::wildcard_imports,
	clippy::all,
	clippy::nursery,
	trivial_casts,
	trivial_numeric_casts,
	missing_docs,
)]
#![doc = include_str!("../README.md")]
// https://matrix.org/docs/matrix-concepts/elements-of-matrix <<-- replace matrix sdk
// https://github.com/matrix-org/matrix-rust-sdk
// @username:example.com for example @sclorentz:matrix.org
// https://spec.matrix.org/v1.17/client-server-api
// https://github.com/matrix-org/matrix-spec

pub struct MTError {
	/// "The errcode string will be a unique string which can be used to handle an error message e.g. M_FORBIDDEN"
	/// <https://spec.matrix.org/v1.17/client-server-api/>
	errcode: String,
	error: String,
}

#[derive(Error, Debug)]
pub enum MTErrorMessages<T> {
	StateStore,
	EventCacheStore,
	EventCacheLock,
	InvalidReceiveMembersParameters,
	DeserializationError,
	Other(T)
}

pub struct MTContent {
	alias: String,
	alt_aliases: Vec<String>
}

pub struct MTEvent {
	pub content: MTContent,
	pub event_id: String,
	/// Timestamp
	pub origin_server_ts: i64,
	pub room_id: String,
	pub sender: String,
	state_key: String,
	type: String,
	// Unsigned events --> "unsigned": {...}
	age: i32,
	prev_content: String,
	redacted_because: String,
	transaction_id: String,
	membership: String,
}

pub struct MTStrippedStateEvent {
	pub content: MTContent,
	pub sender: String,
	state_key: String,
	type: String,
}
