use ash::vk::{self, SurfaceKHR, RenderPass};
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
use std::{error::Error, ptr::NonNull};
use log::debug;
use core::ffi::c_void;

#[derive(Clone, PartialEq, Debug)]
pub enum SurfaceBackend {
	MacOS { ns_view: *mut c_void },
	Windows,
	Linux,
	Headless,
}

#[allow(unused)]
pub struct Renderer {
	pub surface: SurfaceKHR,
	pub renderpass: RenderPass,
	device: ash::Device,
}

/// The rendring interface
impl Renderer {
	/// Detects the phisical device
	fn get_device(instance: &ash::Instance) -> Result<ash::Device, Box<dyn Error>>
	{ unsafe {
		let physical_devices = instance.enumerate_physical_devices()?;
		let physical_device = physical_devices[0];

		let queue_families =
			instance.get_physical_device_queue_family_properties(physical_device);

		let graphics_queue_index = queue_families
			.iter()
			.enumerate()
			.find(|(_, q)| q.queue_flags.contains(vk::QueueFlags::GRAPHICS))
			.map(|(i, _)| i)
			.expect("no graphics queue found") as u32;

		let queue_priority = 1.0;
		let binding = [queue_priority];

		let queue_info = vk::DeviceQueueCreateInfo::default()
			.queue_family_index(graphics_queue_index)
			.queue_priorities(&binding);

		let device_features = vk::PhysicalDeviceFeatures::default();

		let device_create_info = vk::DeviceCreateInfo::default()
			.queue_create_infos(std::slice::from_ref(&queue_info))
			.enabled_features(&device_features);

		let device_extensions = [
			vk::KHR_SWAPCHAIN_NAME.as_ptr(),
		];

		let device_create_info = device_create_info
			.enabled_extension_names(&device_extensions);

		Ok(
			instance.create_device(physical_device, &device_create_info, None)?
		)
	}}

	/// Gets what it's needed to the renderer work
	/// For example, MacOS with EXT_METAL_SURFACE_NAME, because it doesn't have a native vulkan renderer
	fn detect_needed_extensions() -> Vec<*const i8>
	{Vec::from([
		vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
		vk::KHR_SURFACE_NAME.as_ptr(),
		vk::EXT_METAL_SURFACE_NAME.as_ptr(),
	])}

	/// Creates a new Vulkan render
	pub fn new(surface_backend: SurfaceBackend) -> Result<Renderer, Box<dyn Error>>
	{
		let mut view: *mut c_void = std::ptr::null_mut();

		match surface_backend {
			SurfaceBackend::Headless => return Err(
				Box::new("Headless not implemented yet".parse::<u32>().unwrap_err())
			),
			SurfaceBackend::MacOS { ns_view } => {
				view = ns_view;
			},
			_ => {}
		}

		debug!("Creating new vulkan render");

		unsafe {
			let entry = ash::Entry::linked();
			let surface_extensions = Self::detect_needed_extensions();

			let app_info =
				vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));

			let instance_desc = vk::InstanceCreateInfo::default()
				.application_info(&app_info)
				.enabled_extension_names(&surface_extensions);

			let instance = entry.create_instance(&instance_desc, None)?;
			let display_handle = AppKitDisplayHandle::new();

			let nn = NonNull::new(view)
					.expect("NSView is shouldn't be null");
			let window_handle = AppKitWindowHandle::new(nn.cast());

			let device = Self::get_device(&instance)?;
			let renderpass = Self::render_pass(&device)?;

			let surface = ash_window::create_surface(
				&entry,
				&instance,
				raw_window_handle::RawDisplayHandle::AppKit(display_handle),
				raw_window_handle::RawWindowHandle::AppKit(window_handle),
				None
			)?;

			Ok(Renderer {
				surface,
				device,
				renderpass,
			})
		}
	}

	/// Creates a new vulkan renderpass
	/// here's an oficial example: https://github.com/ash-rs/ash/blob/master/ash-examples/src/bin/texture.rs
	pub fn render_pass(device: &ash::Device) -> Result<RenderPass, Box<dyn Error>>
	{
		let renderpass_attachments = [
			vk::AttachmentDescription {
				samples: vk::SampleCountFlags::TYPE_1,
				load_op: vk::AttachmentLoadOp::CLEAR,
				store_op: vk::AttachmentStoreOp::STORE,
				final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
				..Default::default()
			},
			vk::AttachmentDescription {
				format: vk::Format::D16_UNORM,
				samples: vk::SampleCountFlags::TYPE_1,
				load_op: vk::AttachmentLoadOp::CLEAR,
				initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
				final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
				..Default::default()
			},
		];

		let color_attachment_refs = [vk::AttachmentReference {
			attachment: 0,
			layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
		}];
		let depth_attachment_ref = vk::AttachmentReference {
			attachment: 1,
			layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
		};
		let dependencies = [vk::SubpassDependency {
			src_subpass: vk::SUBPASS_EXTERNAL,
			src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
			dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
				| vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
			dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
			..Default::default()
		}];

		let subpass = vk::SubpassDescription::default()
			.color_attachments(&color_attachment_refs)
			.depth_stencil_attachment(&depth_attachment_ref)
			.pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

		let renderpass_create_info = vk::RenderPassCreateInfo::default()
			.attachments(&renderpass_attachments)
			.subpasses(std::slice::from_ref(&subpass))
			.dependencies(&dependencies);

		let renderpass = unsafe {
			device
				.create_render_pass(&renderpass_create_info, None)
				.unwrap()
		};

		return Ok(renderpass);
	}

	// for now returns a generic value
	pub fn get_surface_size(&self) -> (f32, f32) { (0.0, 0.0) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_vulkan_render()
	{
		// for now this should fail, bc I didn't implement the Headless execution
		let renderer = Renderer::new(SurfaceBackend::Headless);
		assert!(renderer.is_ok());
	}
}
