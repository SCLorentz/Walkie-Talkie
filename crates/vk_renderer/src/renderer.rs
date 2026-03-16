#![no_std]
#![allow(clippy::tabs_in_doc_comments, unused_doc_comments)]
#![doc = include_str!("../README.md")]

use ash::{Instance, Device, vk};
use log::debug;
#[allow(unused)]
use core::{slice, ptr::NonNull, error::Error};
use dirty::{Box, void, f8, SurfaceWrapper, Vec};

mod wrapper;
use wrapper::Wrapper;

/// Default Renderer struct
#[allow(missing_docs)]
pub struct Renderer {
	pub device: Device,
	pub instance: Instance,
	data: AppData,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct AppData {
	pub validation: bool,
	// Surface
	pub surface: vk::SurfaceKHR,
	// Physical Device / Logical Device
	pub physical_device: vk::PhysicalDevice,
	pub graphics_queue: vk::Queue,
	pub present_queue: vk::Queue,
	// Swapchain
	pub swapchain_format: vk::Format,
	pub swapchain_extent: vk::Extent2D,
	pub swapchain: vk::SwapchainKHR,
	pub swapchain_images: Vec<vk::Image>,
	pub swapchain_image_views: Vec<vk::ImageView>,
	// Pipeline
	pub render_pass: vk::RenderPass,
	pub pipeline_layout: vk::PipelineLayout,
	pub pipeline: vk::Pipeline,
	// Command Pool
	pub command_pool: vk::CommandPool,
	// Framebuffers
	pub framebuffers: Vec<vk::Framebuffer>,
	// Command Buffers
	pub command_buffers: Vec<vk::CommandBuffer>,
	// Sync Objects
	pub image_available_semaphores: Vec<vk::Semaphore>,
	pub render_finished_semaphores: Vec<vk::Semaphore>,
	pub in_flight_fences: Vec<vk::Fence>,
	pub images_in_flight: Vec<vk::Fence>,
}

/// The rendring interface
/// <https://kylemayes.github.io/vulkanalia/introduction.html>
/// <https://vulkan-tutorial.com/Introduction>
impl Renderer {
	/// Creates a new Vulkan render
	/// this will be our `initVulkan()` from the tutorial
	/// <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code#:~:text=initVulkan()>
	#[allow(clippy::missing_errors_doc)]
	pub fn new(surface_backend: *mut void) -> Result<Self, Box<dyn Error>>
	{
		debug!("Creating new vulkan render");
		let (instance, entry) = Self::create_instance()?;
		let backend: Wrapper = void::from_handle(surface_backend);
		let mut data = AppData::default();

		let device = Self::get_device(&instance)?;
		data.render_pass = Self::render_pass(&device)?;

		Self::create_surface(data.clone(), backend, instance.clone(), entry)?;
		unsafe { Self::create_framebuffers(&device, &mut data)? };

		Ok(Self {
			device,
			instance,
			data
		})
	}

	fn create_instance() -> Result<(Instance, ash::Entry), Box<dyn Error>>
	{
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
		 * `VkApplicationInfo appInfo{};`
		 * for some reason on target linux-a64 it expects "u8" and not "i8" idk y
		 */
		let app_info = vk::ApplicationInfo::default()
			.api_version(vk::make_api_version(0, 1, 0, 0));

		let extensions: &[*const f8] = &[
			vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
			vk::KHR_SURFACE_NAME.as_ptr(),
			#[cfg(target_os = "macos")]
			vk::EXT_METAL_SURFACE_NAME.as_ptr()
		];

		let instance_desc = vk::InstanceCreateInfo::default()
			.application_info(&app_info)
			.enabled_extension_names(extensions);

		let instance: Instance = unsafe {
			/**
			 * `vkCreateInstance(&createInfo, nullptr, &instance)` -> function from C++
			 * in this case, createInfo is `instance_desc`
			 * the Pointer to the variable that stores the handle to the new object (&instance) value isn't necessary
			 *
			 * `VK_ERROR_INCOMPATIBLE_DRIVER`:
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
		let Some(nn_view) =
			NonNull::new(backend.ns_view)
		else
			{ return Err(Box::from("view shouldn't be null")) };

		#[cfg(target_os = "macos")]
		let surface = Self::new_surface(&instance, &entry, nn_view.cast())?;

		#[cfg(target_os = "linux")]
		let surface = Self::new_surface(&instance, &entry, backend.wl_surface, backend.wl_display)?;

		#[cfg(target_os = "windows")]
		let view: *mut void = todo!();

		Ok(Renderer {
			surface,
			renderpass,
			device,
			instance,
		})
	}

	/// Returns the Wrapper for the `SurfaceKHR`
	#[must_use]
	pub fn get_surface(&self) -> SurfaceWrapper
		{ SurfaceWrapper::new(self.data.surface) }

	/**
	 * in the future, implement a rate to pick the best device avaliable
	 * get the device properties and check it can be used
	 * <https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/03_Physical_devices_and_queue_families.html>
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families#:~:text=isDeviceSuitable>
	 */
	#[inline]
	fn is_device_suitable(instance: &Instance, device: vk::PhysicalDevice) -> bool
	{
		let prop = unsafe { instance.get_physical_device_properties(device) };
		let feat = unsafe { instance.get_physical_device_features(device) };

		/**
		 * `INTEGRATED_GPU` will be the case on `MacOS`
		 * for some reason the constant value for that is not working, so im using `.as_raw() == 1`
		 * "associated item not found in `PhysicalDevice`"
		 */
		if feat.geometry_shader == dirty::TRUE
			&& prop.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
			|| prop.device_type.as_raw() == 1 // this represents Self(1) or INTEGRATED_GPU
		{ return true; }

		false
	}

	/**
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families>
	 * this will get the first graphics card avaliable
	 */
	fn get_device(instance: &Instance) -> Result<Device, Box<dyn Error>>
	{
		/**
		 * C++
		 * `uint32_t deviceCount = 0;`
		 * `vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);`
		 * same idea from ours, but ash offers less control, no need to implement verifications of support
		 */
		let physical_devices = unsafe { instance.enumerate_physical_devices()? };

		let mut maybe_selected_device: Option<vk::PhysicalDevice> = None;
		for device in physical_devices {
			if !Self::is_device_suitable(instance, device) { continue }
			debug!("found suitable device `{device:?}`!");
			maybe_selected_device = Some(device);
			break
		}

		let Some(selected_device) = maybe_selected_device else {
			return Err(Box::from("failed to find a suitable GPU!"))
		};

		let queue_families = unsafe {
			instance.get_physical_device_queue_family_properties(selected_device)
		};

		let graphics_queue_index: u32 = u32::try_from(
			queue_families
				.iter()
				.enumerate()
				.find(|(_, q)| q.queue_flags.contains(vk::QueueFlags::GRAPHICS))
				.ok_or("no graphics queue found")?
				.0
		)?;

		let queue_priority = 1.0;
		let binding = [queue_priority];

		let queue_info = vk::DeviceQueueCreateInfo::default()
			.queue_family_index(graphics_queue_index)
			.queue_priorities(&binding);

		let device_features = vk::PhysicalDeviceFeatures::default();

		let device_extensions = [
			vk::KHR_SWAPCHAIN_NAME.as_ptr(),
		];

		let device_create_info = vk::DeviceCreateInfo::default()
			.enabled_extension_names(&device_extensions)
			.queue_create_infos(slice::from_ref(&queue_info))
			.enabled_features(&device_features);

		Ok(unsafe {
			instance.create_device(selected_device, &device_create_info, None)?
		})
	}

	fn create_surface(mut data: AppData, backend: Wrapper, instance: Instance, entry: ash::Entry)
		-> Result<(), Box<dyn Error>>
	{
		/** <https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs>
		 * The Headless backend will be used to implement tests
		 * the repo uses `let surface_loader = surface::Instance::load(&entry, &instance);`
		 * but the way it is created is different, using `SurfaceFactory`, for that I would need winit
		 */

		#[cfg(target_os = "macos")]
		let Some(nn_view) =
			NonNull::new(backend.ns_view)
		else
			{ return Err(Box::from("view shouldn't be null")) };

		#[cfg(target_os = "macos")]
		data.surface = Self::new_surface(&instance, &entry, nn_view.cast())?;

		#[cfg(target_os = "linux")]
		let surface = Self::mew

		Ok(())
	}

	/**
	 * Creates a new surface for `MacOS`
	 *
	 * # Errors
	 *
	 * The function can error in two ways, on the `CALayer` creation, resulting in "failed making the view layer-backed";
	 * Or it can fail on the `create_metal_surface()` method, returning a generic error from ash.
	 */
	#[cfg(target_os = "macos")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		window: NonNull<void>
	) -> Result<vk::SurfaceKHR, Box<dyn Error>>
	{
		use objc2::{rc::Retained, msg_send, runtime::NSObject};
		use objc2_quartz_core::CALayer;
		use ash::ext::metal_surface;
		use core::ffi::c_void; // if only I could use `void::to_handle(val)`...
		debug!("creating metal surface");

		let ns_view: &NSObject = void::from_handle(window.as_ptr());
		let _: () = unsafe { msg_send![ns_view, setWantsLayer: true] };

		let layer: *mut c_void =
			match unsafe { msg_send![ns_view, layer] } {
				Some(val) => Retained::<CALayer>::as_ptr(&val) as *mut c_void,
				None => return Err(Box::from("failed making the view layer-backed"))
			};

		let surface = metal_surface::Instance::new(entry, instance);
		let surface_desc = vk::MetalSurfaceCreateInfoEXT::default()
			.layer(layer); // <- the rust mf expects `*mut c_void` and not the virtually identical `*mut void`

		Ok(unsafe { surface.create_metal_surface(&surface_desc, None)? })
	}

	// WARN: this is just a model and is not complete. The code will fail.
	/// Creates a new surface
	#[cfg(target_os = "linux")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		wl_surface: *mut void,
		wl_display: *mut void,
	) -> Result<SurfaceKHR, Box<dyn Error>>
	{
		debug!("creating linux wayland surface");
		use ash::{khr::wayland_surface, vk::wl_display};

		/**
		 * https://docs.rs/ash-window/0.13.0/src/ash_window/lib.rs.html#36-126
		 */
		let display = wl_display as *mut wl_display;

		let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
			.display(display)
			.surface(wl_surface as *mut core::ffi::c_void);

		let surface = wayland_surface::Instance::new(entry, instance);
		let result = unsafe { surface.create_wayland_surface(&surface_desc, None)? };

		Ok(result)
	}

	/// Creates a new surface
	#[cfg(target_os = "windows")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		window: NonNull<void>
	) -> Result<SurfaceKHR, Box<dyn Error>>
	{
		debug!("creating windows surface");
		use ash::vk::KhrWin32SurfaceExtensionInstanceCommands;

		let surface = instance.create_win32_surface_khr(&info, None).unwrap();
		pick_physical_device(&instance, &mut data)?;

		Ok(result)
	}

	/**
	 * Creates a new vulkan renderpass.
	 *
	 * here's an oficial example: <https://github.com/ash-rs/ash/blob/master/ash-examples/src/bin/texture.rs>
	 */
	#[allow(clippy::missing_errors_doc)]
	pub fn render_pass(device: &Device) -> Result<vk::RenderPass, Box<dyn Error>>
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
			match device
				.create_render_pass(&renderpass_create_info, None) {
					Ok(d) => d,
					Err(e) => return Err(Box::new(e)),
				}
		};

		Ok(renderpass)
	}

	fn create_pipeline(_device: &Device, _data: &mut AppData)
		-> Result<(), Box<dyn Error>>
	{
		Ok(())
	}

	/// <https://kylemayes.github.io/vulkanalia/drawing/framebuffers.html>
	unsafe fn create_framebuffers(device: &Device, data: &mut AppData)
		-> Result<(), Box<dyn Error>>
	{
		data.framebuffers = data
			.swapchain_image_views
			.iter()
			.map(|i| {
				use vk::FramebufferCreateInfo;
				let attachments = &[*i];

				let create_info	= FramebufferCreateInfo::default()
					.render_pass(data.render_pass)
					.attachments(attachments)
					.width(data.swapchain_extent.width)
					.height(data.swapchain_extent.height)
					.layers(1);

				let result = unsafe { device.create_framebuffer(&create_info, None)? };
				Ok(result)
			})
			.collect::<Result<Vec<_>, Box<dyn Error>>>()?;

		Ok(())
	}

	// for now returns a generic value
	/// Returns the surface size
	#[must_use]
	#[allow(clippy::nursery)]
	pub fn get_surface_size(&self) -> (f32, f32) { (0.0, 0.0) }

	/// Stop the rendering and cleanup everything
	unsafe fn cleanup(&self) { unsafe
	{
		// https://docs.rs/ash/latest/ash/khr/surface/struct.InstanceFn.html#structfield.destroy_surface_khr
		//self.instance.destroy_surface_khr(self.surface, None); <- this is type Instance and not InstanceFn
		self.data.framebuffers
			.iter()
			.for_each(|f| self.device.destroy_framebuffer(*f, None));

		self.instance.destroy_instance(None);
	}}

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
