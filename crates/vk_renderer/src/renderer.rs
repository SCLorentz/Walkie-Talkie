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

use ash::Instance;
use ash::vk::{self, SurfaceKHR, RenderPass, PhysicalDevice};
use log::debug;
use core::{slice, error::Error};
use dirty::{Box, void, f8, SurfaceWrapper, Vec};

#[cfg(target_os = "macos")]
use dirty::ptr::NonNull;

mod wrapper;
use wrapper::Wrapper;

#[cfg(target_os = "windows")]
compile_error!("no windows (NT) support");

/*https://raw.githubusercontent.com/ash-rs/ash/master/ash-examples/src/bin/triangle.rs*/

/// Default Renderer struct
#[allow(dead_code)]
pub struct Renderer {
	/// Vulkan Surface
	surface: SurfaceKHR,
	renderpass: RenderPass,
	device: ash::Device,
	instance: Instance,
	framebuffers: vk::Framebuffer,
	image_views: vk::ImageView,
	swapchain: vk::SwapchainKHR,
	swapchain_loader: ash::khr::swapchain::Device,
}

/// The rendring interface
impl Renderer {
	/// Creates a new Vulkan render
	/// this will be our `initVulkan()` from the tutorial
	/// <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code#:~:text=initVulkan()>
	pub fn new(surface_backend: *mut void) -> Result<Renderer, Box<dyn Error>>
	{
		/**
		 * load vulkan in execution, otherwise one might have a problem compiling it for macos (apple beeing apple)
		 * another problem is the inexistence of a vulkan dylib natively on mac
		 * the solution for that problem is packaging the necessary files (dylib) inside the .app
		 * <https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust>
		 */
		let backend: Wrapper = void::from_handle(surface_backend);
		debug!("Creating new vulkan render");

		let entry = unsafe { ash::Entry::load()? };
		let instance = Self::create_instance(&entry)?;

		/** <https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs>
		 * The Headless backend will be used to implement tests
		 * the repo uses `let surface_loader = surface::Instance::load(&entry, &instance);`
		 * but the way it is created is different, using `SurfaceFactory`, for that I would need winit
		 */
		#[cfg(target_os = "macos")]
		let surface = Self::new_surface(&instance, &entry,  NonNull::new(backend.ns_view)?.cast())?;

		#[cfg(target_os = "linux")]
		let surface = Self::new_surface(&instance, &entry, backend.wl_surface, backend.wl_display)?;

		let (device, physical_device) = Self::get_device(&instance)?;

		let (swapchain_loader, swapchain, images, format, extent) =
			Self::create_swapchain(
				&entry,
				&instance,
				&device,
				physical_device,
				surface
			)?;

		let image_views = Self::create_image_views(&device, &images, format)?;
		let renderpass = Self::render_pass(&device)?;
		let framebuffers =
			Self::create_framebuffers(&device, renderpass, &image_views, extent)?;

		Ok(Renderer {
			instance,
			surface,
			device,
			image_views,
			renderpass,
			framebuffers,
			swapchain,
			swapchain_loader
		})
	}

	/**
	 * Create Instance
	 * <https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Instance>
	 * `VkApplicationInfo appInfo{};`
	 * for some reason on target linux-a64 it expects "u8" and not "i8" idk y
	 */
	fn create_instance(entry: &ash::Entry) -> Result<Instance, Box<dyn Error>>
	{
		let app_info = vk::ApplicationInfo::default()
			.api_version(vk::make_api_version(0, 1, 0, 0));

		let extensions: &[*const f8] = &[
			vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr(),
			vk::KHR_SURFACE_NAME.as_ptr(),

			#[cfg(target_os = "macos")]
			vk::EXT_METAL_SURFACE_NAME.as_ptr(),

			#[cfg(target_os = "linux")]
			vk::KHR_WAYLAND_SURFACE_NAME.as_ptr(),
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

		Ok(instance)
	}

	/// Returns the Wrapper for the `SurfaceKHR`
	pub fn get_surface(&self) -> SurfaceWrapper
		{ SurfaceWrapper::new(self.surface) }

	/**
	 * in the future, implement a rate to pick the best device avaliable
	 * get the device properties and check it can be used
	 * <https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/03_Physical_devices_and_queue_families.html>
	 * <https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Physical_devices_and_queue_families#:~:text=isDeviceSuitable>
	 */
	#[inline]
	fn is_device_suitable(instance: &Instance, device: PhysicalDevice) -> bool
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
	fn get_device(instance: &Instance) -> Result<(ash::Device, PhysicalDevice), Box<dyn Error>>
	{
		/**
		 * C++
		 * `uint32_t deviceCount = 0;`
		 * `vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);`
		 * same idea from ours, but ash offers less control, no need to implement verifications of support
		 */
		let physical_devices = unsafe { instance.enumerate_physical_devices()? };

		let mut maybe_selected_device: Option<PhysicalDevice> = None;
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

		Ok((
			unsafe {
				instance.create_device(selected_device, &device_create_info, None)?
			},
			selected_device
		))
	}

	#[allow(unused_variables)]
	fn create_swapchain(
		entry: &ash::Entry,
		instance: &Instance,
		device: &ash::Device,
		physical_device: PhysicalDevice,
		surface: SurfaceKHR,
	) -> Result<
		(
			ash::khr::swapchain::Device,
			vk::SwapchainKHR,
			Vec<vk::Image>,
			vk::Format,
			vk::Extent2D
		),
		Box<dyn Error>
	> {
		todo!();
	}

	#[allow(unused_variables)]
	fn create_framebuffers(
		device: &ash::Device,
		renderpass: RenderPass,
		image_views: &vk::ImageView,
		extent: vk::Extent2D,
	) -> Result<
		vk::Framebuffer,
		Box<dyn Error>
	> {
		todo!();
	}

	#[allow(unused_variables)]
	fn create_image_views(
		device: &ash::Device,
		images: &Vec<vk::Image>,
		format: vk::Format,
	) -> Result<
		vk::ImageView,
		Box<dyn Error>
	> {
		todo!();
	}

	/// Creates a new surface (wayland/metal)
	#[cfg(target_os = "macos")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		window: NonNull<void>
	) -> Result<SurfaceKHR, Box<dyn Error>>
	{
		use objc2::{rc::Retained, msg_send, ClassType};
		use objc2_quartz_core::CALayer;
		use objc2_foundation::NSObject;
		use ash::ext::metal_surface;
		debug!("creating metal surface");

		let ns_view: &NSObject = void::from_handle(window.as_ptr());
		let _: () = unsafe { msg_send![ns_view, setWantsLayer: true] };

		let Some(layer_some): Option<Retained<CALayer>> = (
			unsafe { msg_send![ns_view, layer] }
		) else {
			return Err(Box::from("failed making the view layer-backed"))
		};
		let layer = void::to_handle(&mut layer_some.as_super());

		let surface_desc = vk::MetalSurfaceCreateInfoEXT::default()
			.layer(layer.cast::<core::ffi::c_void>());
		let surface = metal_surface::Instance::new(entry, instance);

		let Ok(metal_surface) = (
			unsafe { surface.create_metal_surface(&surface_desc, None) }
		) else {
			return Err(Box::from("couldn't create metal surface"))
		};

		Ok(metal_surface)
	}

	/// Creates a new surface (wayland/metal)
	#[cfg(target_os = "linux")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		wl_surface: *mut void,
		wl_display: *mut void,
	) -> Result<SurfaceKHR, Box<dyn Error>>
	{
		debug!("creating wayland surface");
		use ash::{khr::wayland_surface, vk::wl_display};

		/**
		 * https://docs.rs/ash-window/0.13.0/src/ash_window/lib.rs.html#36-126
		 */
		let display = wl_display as *mut wl_display;

		let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
			.display(display)
			.surface(wl_surface as *mut core::ffi::c_void);

		let surface = wayland_surface::Instance::new(entry, instance);

		let Ok(wayland_surface) = (
			unsafe { surface.create_wayland_surface(&surface_desc, None) }
		) else {
			return Err(Box::from("couldn't create wayland surface"))
		};

		Ok(wayland_surface)
	}

	/// Creates a new surface
	#[cfg(target_os = "windows")]
	fn new_surface(
		instance: &Instance,
		entry: &ash::Entry,
		window: NonNull<void>
	) -> Result<SurfaceKHR, Box<dyn Error>>
	{
		todo!();
	}

	/// Creates a new vulkan renderpass
	/// here's an oficial example: <https://github.com/ash-rs/ash/blob/master/ash-examples/src/bin/texture.rs>
	pub fn render_pass(device: &ash::Device) -> Result<RenderPass, Box<dyn Error>>
	{
		debug!("creating renderpass");

		let color_attachment = vk::AttachmentDescription::default()
			.format(vk::Format::UNDEFINED) // <-- what is this how can I set a value to it??
			.samples(vk::SampleCountFlags::TYPE_1)
			.load_op(vk::AttachmentLoadOp::CLEAR)
			.store_op(vk::AttachmentStoreOp::STORE)
			.initial_layout(vk::ImageLayout::UNDEFINED)
			.final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

		let color_attachment_ref = vk::AttachmentReference {
			attachment: 0,
			layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
		};

		let dependency = vk::SubpassDependency::default()
			.src_subpass(vk::SUBPASS_EXTERNAL)
			.dst_subpass(0)
			.src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
			.dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
			.dst_access_mask(
				vk::AccessFlags::COLOR_ATTACHMENT_WRITE
			);

		let subpass = vk::SubpassDescription::default()
			.pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
			.color_attachments(&dirty::slice::from_ref(&color_attachment_ref));

		let attachments = [color_attachment];

		let renderpass_create_info = vk::RenderPassCreateInfo::default()
			.attachments(&attachments)
			.subpasses(slice::from_ref(&subpass))
			.dependencies(&dirty::slice::from_ref(&dependency));

		let renderpass = (unsafe {
			device.create_render_pass(&renderpass_create_info, None)
		})?;

		Ok(renderpass)
	}

	// for now returns a generic value
	/// Returns the surface size
	#[must_use]
	pub fn get_surface_size(&self) -> (f32, f32) { (0.0, 0.0) }

	/// Stop the rendering and cleanup everything
	pub fn cleanup(&self)
	{
		unsafe { self.instance.destroy_instance(None); }
	}
}
