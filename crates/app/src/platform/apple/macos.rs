#![allow(unused_imports, unused_doc_comments, clippy::tabs_in_doc_comments)]
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

use crate::{DecorationMode, Decoration, WRequestResult::{self, Fail, Success}, WResponse};

/// Wrapper struct
#[derive(PartialEq, Debug, Clone)]
pub struct Wrapper {
	pub ns_view: *mut void,		// NSView
	pub rect:  *const void,		// NSRect
	pub app:   *const void,		// NSApplication
}

impl Wrapper
{
	fn get<T>(some_ptr: &Retained<T>) -> *mut void where T: Message
	{
		let ptr: *mut T = Retained::<T>::as_ptr(some_ptr).cast_mut();
		ptr.cast()
	}
}

pub trait NativeDecoration
{
	fn run(&self);
	fn new(title: String, width: f64, height: f64) -> WRequestResult<Self> where Self: core::marker::Sized;
	/// Apply blur to window
	fn apply_blur(&mut self) -> WRequestResult<()>;
	/// exit handler
	#[allow(unused)]
	fn exit(&self);
}

impl NativeDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
	fn new(mut title: String, width: f64, height: f64) -> WRequestResult<Self>
	{
		let Some(mtm) = MainThreadMarker::new() else { return Fail(WResponse::UnexpectedError) };

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

		let Some(view) = window.contentView() else { return Fail(WResponse::UnexpectedError) };

		window.center();
		window.setContentMinSize(NSSize::new(width, height));

		let Some(delegate) =
			Delegate::new(window.clone()) else { return Fail(WResponse::UnexpectedError) };
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

		Success(Decoration {
			mode: DecorationMode::ServerSide,
			frame: Wrapper::get(&window),
			backend,
		})
	}

	/// Apply blur effect on the window
	fn apply_blur(&mut self) -> WRequestResult<()>
	{
		let Some(mtm) = MainThreadMarker::new() else { return Fail(WResponse::UnexpectedError) };

		let backend = &raw mut self.backend;
		let rect = unsafe { (*backend).rect.cast::<NSRect>() };

		/**
		 * Blur view configs
		 * Not using liquid glass for this part in specific
		 * Mostly, other effects will be managed trought renderer/shaders on vulkan and not macOS
		 */
		let alloc: Allocated<NSVisualEffectView> = NSVisualEffectView::alloc(mtm);
		let blur_view_ptr = unsafe { NSVisualEffectView::initWithFrame(alloc, *rect) };

		let window_ptr = self.frame as *mut NSWindow;
		let window: &NSWindow = unsafe { &*window_ptr };

		let Some(content) = window.contentView() else {
			log::warn!("couldn't set blur");
			return WRequestResult::Fail(WResponse::UnexpectedError);
		};

		let blur_view = blur_view_ptr.retain();
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
		let app = self.backend.app.cast_mut() as *const NSApplication;
		unsafe { msg_send![&*app, run] }
	}

	fn exit(&self) {}

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
	fn new(window: Retained<NSWindow>) -> Option<Retained<Self>>
	{
		let mtm = MainThreadMarker::new()?;
		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars { window });
		Some(unsafe { msg_send![super(this), init] })
	}
}
