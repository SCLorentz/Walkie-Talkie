use std::cell::OnceCell;
use log::debug;

#[cfg(target_os = "macos")]
use objc2::{
	rc::Retained,
	runtime::ProtocolObject,
	define_class,
	msg_send,
	DefinedClass,
	MainThreadOnly,
};

#[cfg(target_os = "macos")]
use objc2_app_kit::{
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSAutoresizingMaskOptions,
	NSBackingStoreType, NSColor, NSFont, NSTextAlignment, NSTextField, NSWindow, NSWindowDelegate,
	NSWindowStyleMask, NSView
};

#[cfg(target_os = "macos")]
use objc2_foundation::{
	MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
	NSSize, NSString,
};

use crate::{DecorationMode, Decoration};

pub struct CocoaWinDecoration {
	pub mode: DecorationMode,
	pub view: Retained<NSView>,
	pub window: Retained<NSWindow>,
	app: Retained<NSApplication>,
}

pub trait CocoaDecoration
{
	fn run(&self);

	#[cfg(target_os = "macos")]
	fn get_view(&self) -> &NSView;

	#[cfg(target_os = "macos")]
	fn new(mtm: MainThreadMarker, title: &str, width: f64, height: f64) -> Decoration;
}

impl CocoaDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
	#[cfg(target_os = "macos")]
	fn new(mtm: MainThreadMarker, title: &str, width: f64, height: f64) -> Decoration
	{
		debug!("Creating CocoaDecoration object");
		let window = unsafe {
			NSWindow::initWithContentRect_styleMask_backing_defer(
				NSWindow::alloc(mtm),
				NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(width, height)),
				NSWindowStyleMask::Titled
					| NSWindowStyleMask::Closable
					| NSWindowStyleMask::Miniaturizable
					| NSWindowStyleMask::Resizable,
				NSBackingStoreType::Buffered,
				false,
			)
		};

		unsafe { window.setReleasedWhenClosed(false) };

		let ns_title = NSString::from_str(title);
		window.setTitle(&ns_title);

		let view = window.contentView().expect("window must have content view");
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");

		let origin = NSPoint::new(10.0, -2.3);
		let size = NSSize::new(5.0, 0.0);
		let rect = NSRect::new(origin, size);

		window.center();
		window.setContentMinSize(NSSize::new(width, height));

		let delegate = Delegate::new(mtm);
		window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
		window.makeKeyAndOrderFront(None);

		delegate.ivars().window.set(window.clone()).unwrap();

		let app =  NSApplication::sharedApplication(mtm);
		app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
		#[allow(deprecated)]
		app.activateIgnoringOtherApps(true);
		//app.run();

		return Decoration::Apple(CocoaWinDecoration {
			mode: DecorationMode::ServerSide,
			window: window.into(),
			view,
			app,
		});
	}

	fn run(&self) {
		let app = match self {
			Decoration::Apple(dec) => &dec.app,
			_ => panic!("This shouldn't have happened.."),
		};
		unsafe { msg_send![&*app, run] }
	}

	#[cfg(target_os = "macos")]
	fn get_view(&self) -> &NSView
	{
		match self {
			Decoration::Apple(dec) => &dec.view,
			_ => panic!("This shouldn't have happened..."),
		}
	}

	/*fn set_title(&self, title: &str) {
		let ns = NSString::from_str(title);
		self.window.setTitle(&ns);
	}*/
}

#[cfg(target_os = "macos")]
#[derive(Debug, Default)]
struct AppDelegateIvars {
	window: OnceCell<Retained<NSWindow>>,
}

#[cfg(target_os = "macos")]
define_class!(
	#[unsafe(super = NSObject)]
	#[thread_kind = MainThreadOnly]
	#[ivars = AppDelegateIvars]
	struct Delegate;

	unsafe impl NSObjectProtocol for Delegate {}

	unsafe impl NSApplicationDelegate for Delegate {
		#[unsafe(method(applicationDidFinishLaunching:))]
		fn did_finish_launching(&self, _notification: &NSNotification) {}
	}

	unsafe impl NSWindowDelegate for Delegate {
		#[unsafe(method(windowWillClose:))]
		fn window_will_close(&self, _notification: &NSNotification)
		{
			NSApplication::sharedApplication(self.mtm()).terminate(None);
		}
	}
);

impl Delegate {
	fn new(mtm: MainThreadMarker) -> Retained<Self> {
		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars::default());
		unsafe { msg_send![super(this), init] }
	}
}
