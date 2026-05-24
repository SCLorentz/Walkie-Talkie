mod renderer {
	#[cfg(target_os = "linux")]
	pub use vk_renderer::Renderer;

	#[cfg(target_os = "macos")]
	pub use metal_renderer::Renderer;

	#[cfg(target_os = "windows")]
	pub use dx12_renderer::Renderer;
}

pub fn create_renderer(window: &app::Window) -> renderer::Renderer
{
    renderer::Renderer::new(window.get_backend())
        .expect("Renderer initialization failed")
}
