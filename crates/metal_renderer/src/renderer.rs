#![no_std]
#![deny(
	deprecated,
	rust_2018_idioms,
	clippy::shadow_unrelated,
	unreachable_code,
	unused_imports,
	unused_variables,
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
	clippy::arithmetic_side_effects,
	clippy::float_arithmetic,
	clippy::unwrap_in_result,
	clippy::exit,
	clippy::wildcard_imports,
	missing_docs,
	clippy::all,
)]
#![allow(
	clippy::tabs_in_doc_comments,
	unused_doc_comments
)]
#![doc = include_str!("../README.md")]
// https://github.com/madsmtm/objc2/blob/main/examples/metal/circle/main.rs

use core::error::Error;
use dirty::{void, Box, SurfaceWrapper};

use log::debug;
use objc2::{
	runtime::ProtocolObject,
	rc::Retained,
};

use objc2_foundation::{
	NSAutoreleasePool, NSSize
};
use objc2_quartz_core::{CAMetalLayer, CAMetalDrawable};

use objc2_metal::{
	MTLDevice, MTLCreateSystemDefaultDevice, MTLPixelFormat,
	MTLCommandQueue, MTLLoadAction, MTLStoreAction, MTLClearColor, MTLRenderPassDescriptor,
    MTLCommandBuffer, MTLCommandEncoder
};
use objc2_app_kit::NSView;

#[derive(PartialEq, Debug, Clone)]
#[allow(missing_docs)]
pub struct Wrapper {
    pub view:	*mut void,
    pub rect:	*const void,
    pub app:	*const void,
}

#[allow(unused)]
#[allow(missing_docs)]
pub struct Renderer {
    surface:	*const void,
    device:		Retained<ProtocolObject<dyn MTLDevice>>,
    layer:		Retained<CAMetalLayer>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl Renderer {
	#[allow(missing_docs)]
	pub fn new(surface_backend: *mut void) -> Result<Renderer, Box<dyn Error>>
	{
		let backend: Wrapper = void::from_handle(surface_backend);
		let _pool = unsafe { NSAutoreleasePool::new() };
		debug!("creating new metal renderer");

		let view: &NSView = void::from_handle(backend.view);

		let Some(device) = MTLCreateSystemDefaultDevice() else {
			return Err(Box::from("no metal device found"));
		};
		debug!("Your device is: {}", device.name());

		let Some(queue) = device.newCommandQueue() else { return Err(Box::from("message")) };

		let bounds = view.bounds();

		let layer = CAMetalLayer::new();
			layer.setDevice(Some(&device));
			layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
			layer.setPresentsWithTransaction(false);
			layer.setDrawableSize(NSSize::new(800.0, 600.0));
			layer.setFrame(bounds);

		view.setWantsLayer(true);
		view.setLayer(Some(&layer));
		view.setNeedsDisplay(true);
		view.displayIfNeeded();

		Ok(Self {
			surface: void::to_handle(()),
			device,
			layer,
			queue,
		})
	}

	/// draws on the rendering surface
	pub fn draw(&self)
	{
        let Some(drawable) = self.layer.nextDrawable() else {
        	return;
        };

        let render_pass = MTLRenderPassDescriptor::new();

        let color_attachment = unsafe { render_pass
            .colorAttachments()
            .objectAtIndexedSubscript(0)
        };

        color_attachment.setTexture(Some(&drawable.texture()));
        color_attachment.setLoadAction(MTLLoadAction::Clear);
        color_attachment.setStoreAction(MTLStoreAction::Store);
        color_attachment.setClearColor(
            MTLClearColor {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }
        );

        let Some(command_buffer) = self
            .queue
            .commandBuffer()
        else { return };

        let Some(encoder) = command_buffer
            .renderCommandEncoderWithDescriptor(&render_pass)
        else { return };

        encoder.endEncoding();

        command_buffer.presentDrawable(drawable.as_ref());
        command_buffer.commit();
    }

    #[allow(missing_docs)]
    pub fn get_surface(&self) -> SurfaceWrapper {
        SurfaceWrapper::new(self.surface)
    }
}
