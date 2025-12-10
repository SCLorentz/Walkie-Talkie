//use std::mem;

use std::{
	error::Error,
	//fs,
	//sync::Arc,
	//hash::Hash,
	//ffi::{CStr, CString},
};

#[allow(unused)]
use log::{info, warn, debug};

pub struct Renderer {
	//context
}

impl Renderer {
	pub fn new() -> Renderer
	{
		Renderer {}
	}

	fn init_pipeline(&mut self) -> Result<(), Box<dyn Error>>
	{
		Ok(())
	}
}
