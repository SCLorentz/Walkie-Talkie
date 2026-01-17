#![allow(unused_imports, unused_doc_comments)]

use log::debug;
use crate::{void, String};

use objc2::{
	rc::{Retained, Allocated},
	runtime::ProtocolObject,
	define_class,
	msg_send,
	DefinedClass,
	MainThreadOnly,
	Message,
	ClassType
};

use objc2_app_kit::{
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
	NSBackingStoreType, NSColor, NSFont, NSTextAlignment, NSTextField, NSWindow, NSWindowDelegate,
	NSWindowStyleMask, NSView, NSWindowTitleVisibility, NSVisualEffectBlendingMode,
	NSVisualEffectView, NSVisualEffectMaterial, NSVisualEffectState, NSAutoresizingMaskOptions
};

use objc2_foundation::{
	MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
	NSSize, NSString,
};

use crate::{DecorationMode, Decoration, WRequestResult};

#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub ns_view: *mut void,		// NSView
	pub rect:  *const void,		// NSRect
	pub app:   *const void,		// NSApplication
}

impl Wrapper
{
	fn get<T>(ptr: &Retained<T>) -> *mut void where T: Message
	{
		let ptr: *mut T = Retained::<T>::as_ptr(ptr) as *mut T;
		ptr.cast()
	}
}

pub trait NativeDecoration
{
	fn run(&self);
	fn new(title: String, width: f64, height: f64) -> Self;
	fn apply_blur(&mut self) -> WRequestResult<()>;
}

impl NativeDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
	fn new(mut title: String, width: f64, height: f64) -> Self
	{
		let mtm = MainThreadMarker::new()
			.expect("Process expected to be executed on the Main Thread!");

		let origin = NSPoint::new(10.0, -2.3);
		let size = NSSize::new(width, height);
		let rect = NSRect::new(origin, size);

		let window = unsafe { NSWindow::initWithContentRect_styleMask_backing_defer(
			NSWindow::alloc(mtm),
			rect,
			NSWindowStyleMask::Titled
				| NSWindowStyleMask::Closable
				| NSWindowStyleMask::Miniaturizable
				| NSWindowStyleMask::Resizable
				| NSWindowStyleMask::FullSizeContentView,
			NSBackingStoreType::Buffered,
			false,
		)};

		/**
		 * setting the title here even tought it will not be rendered, bc setTitleVisibility
		 * this may change in the future when the GUI is ready
		 */
		window.setTitle(&NSString::from_str(title.as_str()));

		window.setTitlebarAppearsTransparent(true);
		window.setTitleVisibility(NSWindowTitleVisibility(1));
		window.setBackgroundColor(
			Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.8, 0.5, 0.5, 1.0,)
		));

		window.makeKeyAndOrderFront(None);
		unsafe { window.setReleasedWhenClosed(false) };

		let view = window.contentView().expect("window must have content view");
		let mtm = MainThreadMarker::new().expect("Process must run on the Main Thread!");

		window.center();
		window.setContentMinSize(NSSize::new(width, height));

		let delegate = Delegate::new(window.clone());
		window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
		window.makeKeyAndOrderFront(None);

		//delegate.ivars().window.set(window.clone()).unwrap();

		let app =  NSApplication::sharedApplication(mtm);
		app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
		#[allow(deprecated)]
		app.activateIgnoringOtherApps(true);

		let backend = Wrapper {
			ns_view: Wrapper::get(&view),
			rect: dirty::void::to_handle(&rect),
			app: Wrapper::get(&app),
		};

		debug!("Creating NativeDecoration object");

		Decoration {
			mode: DecorationMode::ServerSide,
			frame: Wrapper::get(&window),
			backend,
		}
	}

	/// Apply blur effect on the window
	fn apply_blur(&mut self) -> WRequestResult<()>
	{
		let mtm = MainThreadMarker::new()
			.expect("Process expected to be executed on the Main Thread!");

		let backend = &mut self.backend as *mut Wrapper;
		let rect = unsafe { (*backend).rect as *const NSRect };

		/**
		 * Blur view configs
		 * Not using liquid glass for this part in specific
		 * Mostly, other effects will be managed trought renderer/shaders on vulkan and not macOS
		 */
		let alloc: Allocated<NSVisualEffectView> = NSVisualEffectView::alloc(mtm);
		let blur_view = unsafe { NSVisualEffectView::initWithFrame(alloc, *rect) };

		let window = self.frame as *mut NSWindow;
		let window: &NSWindow = unsafe { &*window };

		let content = window
				.contentView()
				.unwrap();

		let blur_view = blur_view.retain();
		content.addSubview(&blur_view);

		blur_view.setBlendingMode(NSVisualEffectBlendingMode(0));
		blur_view.setMaterial(NSVisualEffectMaterial::HUDWindow);
		blur_view.setState(NSVisualEffectState::Active);
		blur_view.setFrame(content.bounds());
		blur_view.setTranslatesAutoresizingMaskIntoConstraints(false);
		blur_view.setAutoresizingMask(
			NSAutoresizingMaskOptions::ViewWidthSizable
				| NSAutoresizingMaskOptions::ViewHeightSizable
		);

		debug!("applying blur on NativeDecoration");

		WRequestResult::Success(())
	}

	/// The default function to run the program, since it's required on macOS
	fn run(&self)
	{
		let app = self.backend.app as *mut void as *const NSApplication;
		unsafe { msg_send![&*app, run] }
	}

	/*fn set_title(&self, title: &str) {
		let ns = NSString::from_str(title);
		self.window.setTitle(&ns);
	}*/
}

#[derive(Debug)]
#[allow(dead_code)]
struct AppDelegateIvars {
	window: Retained<NSWindow>,
}

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
			{ NSApplication::sharedApplication(self.mtm()).terminate(None); }
	}
);

impl Delegate {
	fn new(window: Retained<NSWindow>) -> Retained<Self>
	{
		let mtm = MainThreadMarker::new()
			.expect("Process expected to be executed on the Main Thread!");

		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars { window });
		unsafe { msg_send![super(this), init] }
	}
}
