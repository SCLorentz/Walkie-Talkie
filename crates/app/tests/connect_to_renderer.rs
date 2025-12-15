use app::Decoration;
use renderer::Renderer;

#[test]
fn test_vulkan_render() {
	let decoration = Decoration::new();
	let result = Renderer::new(decoration.get_view());
	assert!(result.is_ok());
}
