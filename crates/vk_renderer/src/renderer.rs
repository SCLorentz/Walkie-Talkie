#![no_std]
#![allow(unused_doc_comments)]
#![doc = include_str!("../README.md")]

use ash::Instance;
use ash::vk::{self, SurfaceKHR, RenderPass, PhysicalDevice};
use log::debug;
use core::{slice, ptr::NonNull, error::Error};
use common::{from_handle, Box, void};

mod wrapper;
use wrapper::Wrapper;

#[allow(dead_code)]
pub struct Renderer {
	pub surface: SurfaceKHR,
	renderpass: RenderPass,
	device: ash::Device,
	instance: Instance,
}

/// The rendring interface
impl Renderer {
	/// Creates a new Vulkan render
	/// this will be our initVulkan() from the tutorial
	/// <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code#:~:text=initVulkan()>
	pub fn new(surface_backend: *mut void) -> Result<Renderer, Box<dyn Error>>
	{
		let backend: Wrapper = from_handle(surface_backend);
		debug!("Creating new vulkan render");

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
		let app_info = vk::ApplicationInfo::default()
			.api_version(vk::make_api_version(0, 1, 0, 0));

		let extensions: &[*const i8] = &[
			vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
			vk::KHR_SURFACE_NAME.as_ptr(),
			#[cfg(target_os = "macos")]
			vk::EXT_METAL_SURFACE_NAME.as_ptr(),
		];

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
		//let _logic = Self::create_logic_device();
		let renderpass = Self::render_pass(&device)?;

		/** <https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs>
		 * The Headless backend will be used to implement tests
		 * the repo uses `let surface_loader = surface::Instance::load(&entry, &instance);`
		 * but the way it is created is different, using `SurfaceFactory`, for that I would need winit
		 */

		#[cfg(target_os = "macos")]
		let view = backend.ns_view;
		let nn_view = NonNull::new(view)
			.expect("NSView shouldn't be null")
			.cast();

		let surface = Self::new_surface(&instance, &entry, nn_view);

		Ok(Renderer {
			instance,
			surface,
			device,
			renderpass,
		})
	}
	/**
	 * in the future, implement a rate to pick the best device avaliable
	 * get the device properties and check it can be used
	 * <https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/03_Physical_devices_and_queue_families.html>
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families#:~:text=isDeviceSuitable>
	 */
	#[inline]
	fn is_device_suitable(instance: &ash::Instance, device: PhysicalDevice) -> bool
	{
		let prop = unsafe { instance.get_physical_device_properties(device) };
		let feat = unsafe { instance.get_physical_device_features(device) };

		/**
		 * INTEGRATED_GPU will be the case on MacOS
		 * for some reason the constant value for that is not working, so im using `.as_raw() == 1`
		 * "associated item not found in `PhysicalDevice`"
		 */
		if feat.geometry_shader == common::TRUE
			&& prop.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
			|| prop.device_type.as_raw() == 1 // this represents Self(1) or INTEGRATED_GPU
		{ return true; }

		false
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

		let mut selected_device: Option<PhysicalDevice> = None;
		for device in physical_devices {
			if !Self::is_device_suitable(instance, device) { continue }
			debug!("found suitable device `{:?}`!", device);
			selected_device = Some(device);
			break
		}

		let selected_device = selected_device.unwrap_or_else(||
			panic!("failed to find a suitable GPU!")
		);

		let queue_families =
			unsafe { instance.get_physical_device_queue_family_properties(selected_device) };

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
			.queue_create_infos(slice::from_ref(&queue_info))
			.enabled_features(&device_features);

		let device_extensions = [
			vk::KHR_SWAPCHAIN_NAME.as_ptr(),
		];

		let device_create_info = device_create_info
			.enabled_extension_names(&device_extensions);

		Ok(
			unsafe { instance.create_device(selected_device, &device_create_info, None)? }
		)
	}

	#[cfg(target_os = "macos")]
	fn new_surface(instance: &Instance, entry: &ash::Entry, window: NonNull<void>) -> SurfaceKHR
	{
		use objc2::{rc::Retained, msg_send, ClassType};
		use objc2_quartz_core::CALayer;
		use objc2_foundation::NSObject;
		use ash::ext::metal_surface;
		debug!("creating metal surface");

		let ns_view: &NSObject = unsafe { window.cast().as_ref() };
		let _: () = unsafe { msg_send![ns_view, setWantsLayer: true] };

		let layer: Option<Retained<CALayer>> = unsafe { msg_send![ns_view, layer] };
		let layer = common::to_handle(
			&mut layer.expect("failed making the view layer-backed").as_super()
		);

		let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(layer as *const core::ffi::c_void);
		let surface = metal_surface::Instance::new(entry, instance);
		unsafe {
			surface.create_metal_surface(&surface_desc, None)
				.expect("couldn't create metal surface")
		}
	}

	// WARN: this is just a model and is not complete. The code will fail.
	/*#[cfg(target_os = "linux")]
	fn new_surface(instance: &Instance, entry: &ash::Entry, window: NonNull<void>) -> SurfaceKHR
	{
		debug!("creating linux wayland surface");
		use ash::{khr::wayland_surface, vk::wl_display};

		pub struct WaylandWindowHandle {
			pub surface: NonNull<void>,
		}

		let window: &WaylandWindowHandle = unsafe { window.cast().as_ref() };
		/**
		 * https://docs.rs/ash-window/0.13.0/src/ash_window/lib.rs.html#36-126
		 */
		let display = core::ptr::null_mut() as *const void as *mut wl_display;

		let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
			.display(display)
			.surface((*window).surface.as_ptr());

		let surface = wayland_surface::Instance::new(entry, instance);
		unsafe { surface.create_wayland_surface(&surface_desc, None)
			.expect("couldn't create wayland surface") }
	}*/

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
			.subpasses(slice::from_ref(&subpass))
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
}
