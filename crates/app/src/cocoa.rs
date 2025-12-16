#![allow(unused_imports)]

use std::cell::OnceCell;
use core::ffi::c_void;
use log::debug;
use renderer::SurfaceBackend;
use objc2::Message;

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
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
	NSBackingStoreType, NSColor, NSFont, NSTextAlignment, NSTextField, NSWindow, NSWindowDelegate,
	NSWindowStyleMask, NSView
};

#[cfg(target_os = "macos")]
use objc2_foundation::{
	MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
	NSSize, NSString,
};

use crate::{DecorationMode, Decoration};

#[cfg(target_os = "macos")]
pub trait CocoaDecoration
{
	fn run(&self);
	fn get_view(&self) -> *mut c_void;
	fn new(mtm: MainThreadMarker, title: &str, width: f64, height: f64) -> Decoration;
}

#[cfg(target_os = "macos")]
impl CocoaDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
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

		Decoration {
			mode: DecorationMode::ServerSide,
			frame: to_c_void(&window),
			app: to_c_void(&app),
			backend: SurfaceBackend::MacOS { ns_view: to_c_void(&view), }
		}
	}

	/// The default function to run the program, since it's required on macOS
	fn run(&self)
	{
		let app = self.app as *mut c_void as *const NSView;

		unsafe { msg_send![&*app, run] }
	}

	/// Returns the NSView element from the window
	fn get_view(&self) -> *mut c_void {
		match self.backend {
			SurfaceBackend::MacOS { ns_view: view } => view,
			_ => todo!(),
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

#[cfg(target_os = "macos")]
impl Delegate {
	fn new(mtm: MainThreadMarker) -> Retained<Self> {
		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars::default());
		unsafe { msg_send![super(this), init] }
	}
}

fn to_c_void<T>(ptr: &Retained<T>)
	-> *mut c_void where T: Message
{
	let ptr: *mut T = Retained::<T>::as_ptr(&ptr) as *mut T;
	ptr.cast()
}
