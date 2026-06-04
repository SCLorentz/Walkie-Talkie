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

use core::{error::Error, ptr::NonNull};
use dirty::{void, Box, SurfaceWrapper};

use log::debug;
use objc2::{
	runtime::ProtocolObject,
	rc::Retained,
	MainThreadMarker,
	MainThreadOnly,
};

use objc2_foundation::{
	NSAutoreleasePool, NSSize, ns_string
};
use objc2_quartz_core::CAMetalLayer;

use objc2_metal::{
	MTLCommandBuffer, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary,
	MTLPixelFormat, MTLRenderPipelineState, MTLRenderCommandEncoder, MTLPrimitiveType,
	MTLCommandEncoder,
};
use objc2_metal_kit::MTKView;
use objc2_app_kit::{NSView, NSApplication};

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
    queue:		Retained<ProtocolObject<dyn MTLCommandQueue>>,
    pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    mtk_view:	Retained<MTKView>,
}

impl Renderer {
	#[allow(missing_docs)]
	pub fn new(surface_backend: *mut void) -> Result<Renderer, Box<dyn Error>>
	{
		let backend: Wrapper = void::from_handle(surface_backend);
		let _pool = unsafe { NSAutoreleasePool::new() };
		debug!("creating new metal renderer");

		let Some(mtm) = MainThreadMarker::new() else { return Err(Box::from("not main thread")) };

		let view: &NSView = void::from_handle(backend.view);
		let app = NSApplication::sharedApplication(mtm);
		let window = app.windows().objectAtIndex(0); // suposing at least one window exists

		let Some(device) = MTLCreateSystemDefaultDevice() else {
			return Err(Box::from("no metal device found"));
		};

		debug!("Your device is: {}", device.name());

		let Some(queue) = device.newCommandQueue() else { return Err(Box::from("couldn't create queue")) };

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

		let mtk_view = {
			let frame_rect = window.frame();
			MTKView::initWithFrame_device(MTKView::alloc(mtm), frame_rect, Some(&device))
		};
		window.contentView().unwrap().addSubview(&mtk_view);

		let library = device
            .newLibraryWithSource_options_error(ns_string!(include_str!("triangle.metal")), None)
            .unwrap_or_else(|e| panic!("Failed to create a library: {e}"));

		let pipeline_descriptor = objc2_metal::MTLRenderPipelineDescriptor::new();
		unsafe {
            pipeline_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
                .setPixelFormat(mtk_view.colorPixelFormat());
        }

        let vertex_function = library.newFunctionWithName(ns_string!("vertex_main"));
        pipeline_descriptor.setVertexFunction(vertex_function.as_deref());

        let fragment_function = library.newFunctionWithName(ns_string!("fragment_main"));
        pipeline_descriptor.setFragmentFunction(fragment_function.as_deref());

        let pipeline_state = device
            .newRenderPipelineStateWithDescriptor_error(&pipeline_descriptor)
            .expect("Failed to create a pipeline state.");

        debug!("renderer pipeline created!");

		Ok(Self {
			surface: void::to_handle(()),
			device,
			layer,
			queue,
			pipeline_state,
			mtk_view,
		})
	}

	/// Draws the content based on the vertex and fragment shaders
	pub fn draw(&self)
	{
		debug!("drawing on GPU");
		let mut alpha: f32 = 1.0;

		let drawable = self.mtk_view.currentDrawable().unwrap();
		let descriptor = self.mtk_view.currentRenderPassDescriptor().unwrap();
		let command_buffer = self.queue.commandBuffer().unwrap();
		let encoder = command_buffer.renderCommandEncoderWithDescriptor(&descriptor).unwrap();

		encoder.setRenderPipelineState(&self.pipeline_state);

		let Some(val) = NonNull::new((&mut alpha as *mut f32).cast()) else { return };

		unsafe {
			encoder.setVertexBytes_length_atIndex(
				val,
				core::mem::size_of::<f32>(),
				0,
			);

			encoder.setFragmentBytes_length_atIndex(
				val,
				core::mem::size_of::<f32>(),
				0,
			);

			encoder.drawPrimitives_vertexStart_vertexCount(
				MTLPrimitiveType::Triangle,
				0,
				6,
			);
		}

		encoder.endEncoding();

		command_buffer.presentDrawable(&drawable.as_ref());
		command_buffer.commit();
	}

    #[allow(missing_docs)]
    pub fn get_surface(&self) -> SurfaceWrapper {
        SurfaceWrapper::new(self.surface)
    }
}
