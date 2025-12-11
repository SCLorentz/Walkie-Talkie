#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;

#[cfg(target_os = "macos")]
use objc2::{
	MainThreadOnly,
	MainThreadMarker,
	rc::Retained,
};

#[cfg(target_os = "macos")]
use objc2_foundation::{NSRect, NSPoint, NSSize};

use crate::{DecorationMode, Decoration};

pub struct CocoaWinDecoration {
	pub mode: DecorationMode,
	pub view: Retained<NSView>,
}

pub trait CocoaDecoration
{
	#[cfg(target_os = "macos")]
	#[allow(unused)]
	fn make_view() -> Retained<NSView>;

	#[cfg(target_os = "macos")]
	fn get_view(&self) -> &NSView;

	#[cfg(target_os = "macos")]
	fn new() -> Decoration;
}

impl CocoaDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
	#[cfg(target_os = "macos")]
	fn new() -> Decoration
	{
		return Decoration::Apple(CocoaWinDecoration {
			mode: DecorationMode::ServerSide,
			view: Self::make_view(),
		});
	}

	#[cfg(target_os = "macos")]
	fn make_view() -> Retained<NSView>
	{
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");

		let origin = NSPoint::new(10.0, -2.3);
		let size = NSSize::new(5.0, 0.0);
		let rect = NSRect::new(origin, size);

		NSView::initWithFrame(NSView::alloc(mtm), rect)
	}

	#[cfg(target_os = "macos")]
	fn get_view(&self) -> &NSView
	{
		match self {
			Decoration::Apple(dec) => &dec.view,
			_ => panic!("This shouldn't have happened..."),
		}
	}
}
