#![allow(unused_doc_comments)]
#![doc = include_str!("../README.md")]

use ash::Instance;
use ash::vk::{self, SurfaceKHR, RenderPass, Handle, PhysicalDevice};
use std::{error::Error, ptr::NonNull};
use log::debug;
use core::ffi::c_void;
use common::SurfaceBackend;

#[allow(dead_code)]
pub struct Renderer {
	pub surface: SurfaceKHR,
	pub renderpass: RenderPass,
	device: ash::Device,
	instance: Instance,
}

/// The rendring interface
impl Renderer {
	/**
	 * in the future, implement a rate to pick the best device avaliable
	 * get the device properties and check it can be used
	 * for now returns true always, but more info here:
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families#:~:text=isDeviceSuitable>
	 */
	fn is_device_suitable(_device: PhysicalDevice) -> bool
	{
		true
	}

	/**
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families>
	 * this will get the first graphics card avaliable
	 */
	fn get_device(instance: &ash::Instance) -> Result<ash::Device, Box<dyn Error>>
	{
		/**
		 * C++
		 * uint32_t deviceCount = 0;
		 * vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);
		 * same idea from ours, but ash offers less control, no need to implement verifications of support
		 */
		let physical_devices = unsafe { instance.enumerate_physical_devices()? };
		let physical_device = physical_devices[0];

		/**
		 * this can be null if the function enumerate_physical_devices() didn't find any compatible gpu
		 */
		if physical_device.is_null() == true || !Self::is_device_suitable(physical_device)
		{
			panic!("failed to find a suitable GPU!");
		}

		let queue_families =
			unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

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
			unsafe { instance.create_device(physical_device, &device_create_info, None)? }
		)
	}

	// TODO: make this work (yes, I know it's ugly)
	// For some reason it returns `Vulkan inicialization failed: ERROR_INCOMPATIBLE_DRIVER`
	// it shouldn't happen because of the MoltenVK_icd.json
	/*#[cfg(target_os = "macos")]
	fn vulkan_entry() -> ash::Entry
	{
		use std::env;

		/**
		 * MacOS does not have vulkan natively, so, we need to load the libs ourselves
		 * Get the self.app path and load dependencies packaged inside Resources/ and Frameworks/
		 */
		#[cfg(not(debug_assertions))]
		use std::path::PathBuf;

		#[cfg(not(debug_assertions))]
		let exe = env::current_exe().unwrap();

		#[cfg(not(debug_assertions))]
		let contents = exe.parent()
			.unwrap()
			.parent()
			.unwrap();

		#[cfg(not(debug_assertions))]
		let icd = contents
			.join("Resources/vulkan/icd.d/MoltenVK_icd.json");

		#[cfg(not(debug_assertions))]
		let loader = contents
			.join("Frameworks/libvulkan.dylib");

		/**
		 * Debug
		 * this env wont be inside .app but on target/debug/, so we will need to get the libs from the global install
		 */
		#[cfg(debug_assertions)]
		let icd = "/opt/homebrew/Cellar/molten-vk/1.4.0/etc/vulkan/icd.d/MoltenVK_icd.json";

		#[cfg(debug_assertions)]
		let loader = "/opt/homebrew/lib/libvulkan.dylib";

		unsafe {
			env::set_var("VK_ICD_FILENAMES", icd);

			ash::Entry::load_from(loader)
				.expect("error loading MoltenVK")
		}
	}*/

	#[cfg(target_os = "macos")]
	fn new_surface(instance: &Instance, entry: &ash::Entry, window: NonNull<c_void>) -> SurfaceKHR
	{
		use ash::ext::metal_surface;
		use objc2_quartz_core::CALayer;
		use objc2_foundation::NSObject;
		use objc2::msg_send;
		debug!("creating metal surface");

		let ns_view: &NSObject = unsafe { window.cast().as_ref() };
		let _: () = unsafe { msg_send![ns_view, setWantsLayer: true] };

		let layer: Option<Retained<CALayer>> = unsafe { msg_send![ns_view, layer] };
		let layer = to_c_void(
			&layer.expect("failed making the view layer-backed")
		);

		let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(layer);
		let surface = metal_surface::Instance::new(entry, instance);
		unsafe {
			surface.create_metal_surface(&surface_desc, None)
				.expect("couldn't create metal surface")
		}
	}

	// WARN: this is just a model and is not complete. The code will fail.
	#[cfg(target_os = "linux")]
	fn new_surface(instance: &Instance, entry: &ash::Entry, window: NonNull<c_void>) -> SurfaceKHR
	{
		debug!("creating linux wayland surface");

		use ash::{khr::wayland_surface, vk::wl_display};

		pub struct WaylandWindowHandle {
			pub surface: NonNull<c_void>,
		}

		let window: &WaylandWindowHandle = unsafe { window.cast().as_ref() };
		/**
		 * https://docs.rs/ash-window/0.13.0/src/ash_window/lib.rs.html#36-126
		 */
		let display = std::ptr::null_mut() as *const c_void as *mut wl_display;

		let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
			.display(display)
			.surface((*window).surface.as_ptr());

		let surface = wayland_surface::Instance::new(entry, instance);
		unsafe { surface.create_wayland_surface(&surface_desc, None)
			.expect("couldn't create wayland surface") }
	}

	/// Gets what it's needed to the renderer work
	/// For example, MacOS with `EXT_METAL_SURFACE_NAME`, because it doesn't have a native vulkan renderer
	fn detect_needed_extensions() -> Vec<*const i8>
	{Vec::from([
		vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
		vk::KHR_SURFACE_NAME.as_ptr(),
		vk::EXT_METAL_SURFACE_NAME.as_ptr(),
	])}

	/// Creates a new Vulkan render
	/// this will be our initVulkan() from the tutorial
	/// <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code#:~:text=initVulkan()>
	pub fn new(surface_backend: &mut SurfaceBackend) -> Result<Renderer, Box<dyn Error>>
	{
		// for some reason idk how to take this off without invalid memory address
		let ptr = surface_backend as *mut SurfaceBackend;
		let surface_backend = unsafe { (*ptr).clone() };
		debug!("Creating new vulkan render on backend:\n{:#?}", surface_backend);

		/**
		 * load vulkan in execution, otherwise one might have a problem compiling it for macos (apple beeing apple)
		 * another problem is the inexistence of a vulkan dylib natively on mac
		 * the solution for that problem is packaging the necessary files (dylib) inside the .app
		 * <https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust>
		 */
		let entry = unsafe { ash::Entry::load()? };

		/**
		 * Create Instance
		 * <https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Instance>
		 * VkApplicationInfo appInfo{};
		 */
		let app_info =
			vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));

		let extensions = Self::detect_needed_extensions();
		let instance_desc = vk::InstanceCreateInfo::default()
			.application_info(&app_info)
			.enabled_extension_names(&extensions);

		let instance: Instance = unsafe {
			/**
			 * vkCreateInstance(&createInfo, nullptr, &instance) -> function from C++
			 * in this case, createInfo is instance_desc
			 * the Pointer to the variable that stores the handle to the new object (&instance) value isn't necessary
			 *
			 * VK_ERROR_INCOMPATIBLE_DRIVER:
			 * <https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Instance#:~:text=Encountered%20VK%5FERROR%5FINCOMPATIBLE%5FDRIVER>
			 */
			entry.create_instance(&instance_desc, None)?
		};

		/**
		 * Handlers
		 */
		let device = Self::get_device(&instance)?;
		let renderpass = Self::render_pass(&device)?;

		/** <https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs>
		 * The Headless backend will be used to implement tests
		 * the repo uses `let surface_loader = surface::Instance::load(&entry, &instance);`
		 * but the way it is created is different, using `SurfaceFactory`, for that I would need winit
		 */

		let view = match surface_backend {
			#[cfg(debug_assertions)]
			SurfaceBackend::Headless => todo!(),
			SurfaceBackend::MacOS { ns_view, .. } => ns_view,
			SurfaceBackend::Linux { wayland_view } => wayland_view,
			_ => todo!()
		};

		let nn_view = NonNull::new(view)
			.expect("NSView is shouldn't be null")
			.cast();

		let surface = Self::new_surface(&instance, &entry, nn_view);

		Ok(Renderer {
			instance,
			surface,
			device,
			renderpass,
		})
	}

	/// Creates a new vulkan renderpass
	/// here's an oficial example: <https://github.com/ash-rs/ash/blob/master/ash-examples/src/bin/texture.rs>
	pub fn render_pass(device: &ash::Device) -> Result<RenderPass, Box<dyn Error>>
	{
		// tbh, I have no idea what does this do
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

	#[allow(unused)]
	pub fn cleanup(&self)
	{
		unsafe { self.instance.destroy_instance(None); }
	}
}

/*#[cfg(test)] // this wont work on linux for some reason
mod tests {
	use super::*;

	#[test]
	fn test_vulkan_render()
	{
		// for now this should fail, bc I didn't implement the Headless execution
		let renderer = Renderer::new(SurfaceBackend::Headless as *mut SurfaceBackend as *mut c_void);
		assert!(renderer.is_ok());
	}
}*/

// This is duplicate! Also avaliable on crates/app/cocoa.rs
#[cfg(target_os = "macos")]
use objc2::{rc::Retained, Message};

#[cfg(target_os = "macos")]
fn to_c_void<T>(ptr: &Retained<T>)
	-> *mut c_void where T: Message
{
	let ptr: *mut T = Retained::<T>::as_ptr(&ptr) as *mut T;
	ptr.cast()
}
