#![deny(unsafe_op_in_unsafe_fn)]
use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSAutoresizingMaskOptions,
    NSBackingStoreType, NSColor, NSFont, NSTextAlignment, NSTextField, NSWindow, NSWindowDelegate,
    NSWindowStyleMask, NSView
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSSize,
};

use crate::{DecorationMode, Decoration};

pub struct CocoaWinDecoration {
	pub app: Retained<NSApplication>,
	pub mode: DecorationMode,
	pub view: Retained<NSView>,
}

pub trait CocoaDecoration
{
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
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");

		let origin = NSPoint::new(10.0, -2.3);
		let size = NSSize::new(5.0, 0.0);
		let rect = NSRect::new(origin, size);

		let view = NSView::initWithFrame(NSView::alloc(mtm), rect);

		//let app = NSApplication::sharedApplication(mtm);
		//let delegate = Delegate::new(mtm);
		//app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

		//app.run();

		return Decoration::Apple(CocoaWinDecoration {
			mode: DecorationMode::ServerSide,
			view,
			app,
		});
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

#[derive(Debug, Default)]
struct AppDelegateIvars {
	window: OnceCell<Retained<NSWindow>>,
}

define_class!(
	#[unsafe(super = NSObject)]
	#[thread_kind = MainThreadOnly]
	#[ivars = AppDelegateIvars]
	struct Delegate;

	unsafe impl NSObjectProtocol for Delegate {}

	unsafe impl NSApplicationDelegate for Delegate {
		#[unsafe(method(applicationDidFinishLaunching:))]
		fn did_finish_launching(&self, notification: &NSNotification) {
			let mtm = self.mtm();

			let app = unsafe { notification.object() }
				.unwrap()
				.downcast::<NSApplication>()
				.unwrap();

			let window = unsafe {
				NSWindow::initWithContentRect_styleMask_backing_defer(
					NSWindow::alloc(mtm),
					NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 300.0)), // <<-- use the generated rect on CocoaDecoration::new()
					NSWindowStyleMask::Titled
						| NSWindowStyleMask::Closable
						| NSWindowStyleMask::Miniaturizable
						| NSWindowStyleMask::Resizable,
						NSBackingStoreType::Buffered,
						false,
				)
			};

			// How can I access this??
			// NSString::from_str(&title)??
			window.setTitle(ns_string!("A window")); // <<--- How can I set this name without a constant?
			let view = window.contentView().expect("window must have content view"); // <<--- HERE connect with vulkan
			window.center();
			unsafe { window.setContentMinSize(NSSize::new(300.0, 300.0)) };
			window.setDelegate(Some(ProtocolObject::from_ref(self)));

			window.makeKeyAndOrderFront(None);

			self.ivars().window.set(window).unwrap();

			app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

			#[allow(deprecated)]
			app.activateIgnoringOtherApps(true);
		}
	}

	unsafe impl NSWindowDelegate for Delegate
	{
		#[unsafe(method(windowWillClose:))]
		fn window_will_close(&self, _notification: &NSNotification)
		{
			unsafe { NSApplication::sharedApplication(self.mtm()).terminate(None) };
		}
	}
);

impl Delegate {
	fn new(mtm: MainThreadMarker) -> Retained<Self> {
		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars::default());
		unsafe { msg_send![super(this), init] }
	}
}
