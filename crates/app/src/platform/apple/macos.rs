#![allow(unused_doc_comments, clippy::tabs_in_doc_comments)]
use log::debug;
use crate::{void, String, DecorationMode, Decoration, WResponse, ThemeDefault, NativeDecoration};

use objc2::{
	rc::{Retained, Allocated},
	runtime::ProtocolObject,
	define_class,
	msg_send,
	MainThreadOnly,
	Message,
	sel
};

use objc2_app_kit::{
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
	NSBackingStoreType, NSColor, NSWindow, NSWindowDelegate,
	NSWindowStyleMask, NSView, NSWindowTitleVisibility, NSVisualEffectBlendingMode,
	NSVisualEffectView, NSVisualEffectMaterial, NSVisualEffectState, NSAutoresizingMaskOptions,
	NSMenu, NSMenuItem
};

use objc2_foundation::{
	MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
	NSSize, NSString,
};

/// Wrapper struct
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Wrapper {
	pub ns_view: *mut void,		// NSView
	pub rect:  *const void,		// NSRect
}

impl NativeDecoration for Decoration
{
	/// Creates the native window frame decoration for macOS
	fn new(title: String, width: f64, height: f64, theme: ThemeDefault) -> Result<Self, WResponse>
	{
		let Some(mtm) = MainThreadMarker::new() else { return Err(WResponse::MainThreadError) };

		let rect = NSRect::new(
			NSPoint::new(10.0, -2.3),
			NSSize::new(width, height)
		);

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

		window.setTitle(&NSString::from_str(title.as_str()));

		if !theme.has_title {
			window.setTitlebarAppearsTransparent(true);
			window.setTitleVisibility(NSWindowTitleVisibility(1));
		}

		// TODO: this should be handled by the graphical backend if used with `blur`
		let (r, g, b, a) = theme.background_color.to_default();
		window.setBackgroundColor(
			Some(&NSColor::colorWithSRGBRed_green_blue_alpha(r, g, b, a)
		));

		window.makeKeyAndOrderFront(None);
		unsafe { window.setReleasedWhenClosed(false) };

		let Some(view) = window.contentView() else { return Err(WResponse::UnexpectedError) };

		window.center();
		window.setContentMinSize(NSSize::new(width, height));

		let Some(delegate) =
			Delegate::new(window.clone()) else { return Err(WResponse::UnexpectedError) };

		window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
		window.makeKeyAndOrderFront(None);

		let app = NSApplication::sharedApplication(mtm);
		let _ = app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
		app.activate();

		let backend = Wrapper {
			ns_view: void::to_handle(Retained::<NSView>::as_ptr(&view).cast_mut()),
			rect: void::to_handle(&rect),
		};

		debug!("Creating NativeDecoration object");

		Ok(Self {
			mode: DecorationMode::ServerSide,
			frame: void::to_handle(Retained::<NSWindow>::as_ptr(&window).cast_mut()),
			backend,
		})
	}

	/// Apply blur effect on the window
	fn apply_blur(&mut self) -> Result<(), WResponse>
	{
		let Some(mtm) = MainThreadMarker::new() else { return Err(WResponse::MainThreadError) };

		let backend = self.backend.clone();
		let rect: NSRect = void::from_handle(backend.rect);
		/**
		 * Blur view configs
		 * Not using liquid glass for this part in specific
		 * Mostly, other effects will be managed trought renderer/shaders on vulkan and not macOS
		 */
		let alloc: Allocated<NSVisualEffectView> = NSVisualEffectView::alloc(mtm);
		let window: &NSWindow = void::from_handle(self.frame);

		let Some(content) = window.contentView() else {
			log::warn!("couldn't set blur");
			return Err(WResponse::UnexpectedError);
		};

		let blur = NSVisualEffectView::initWithFrame(alloc, rect).retain();
			blur.setBlendingMode(NSVisualEffectBlendingMode(0));
			blur.setMaterial(NSVisualEffectMaterial::HUDWindow);
			blur.setState(NSVisualEffectState::Active);
			blur.setFrame(content.bounds());
			blur.setTranslatesAutoresizingMaskIntoConstraints(false);
			blur.setAutoresizingMask(
				NSAutoresizingMaskOptions::ViewWidthSizable
					| NSAutoresizingMaskOptions::ViewHeightSizable
			);
		content.addSubview(&blur);

		debug!("applying blur on NativeDecoration");
		Ok(())
	}

	fn create_app_menu(&self, app_name: String) -> Result<(), WResponse>
	{
		let Some(mtm) = MainThreadMarker::new() else { return Err(WResponse::MainThreadError) };
		let app = NSApplication::sharedApplication(mtm);

		let item_menu = NSMenuItem::alloc(mtm);
		let quit_item = unsafe { NSMenuItem::initWithTitle_action_keyEquivalent(
			item_menu,
			&NSString::from_str(dirty::format!("Quit {app_name}").as_str()),
			Some(sel!(terminate:)),
			&NSString::from_str("q")
		) };
		unsafe { quit_item.setTarget(Some(&app)) };

		let app_menu = NSMenu::new(mtm);
			app_menu.addItem(&quit_item);

		let app_menu_item = NSMenuItem::new(mtm);
		let menubar = NSMenu::new(mtm);
			menubar.addItem(&app_menu_item);

		app_menu_item.setSubmenu(Some(&app_menu));
		app.setMainMenu(Some(&menubar));

		debug!("creating app menu");
		Ok(())
	}

	/// The default function to run the program, since it's required on macOS
	fn run(&self)
	{
		use objc2::{class, runtime::AnyObject};
		let raw: *mut NSApplication = unsafe {
			msg_send![class!(NSApplication), sharedApplication]
		};

		unsafe { let _: *mut AnyObject = msg_send![raw, retain]; }
		let Some(app) = (unsafe { Retained::from_raw(raw) }) else {
			log::error!("no NSApplication found");
			return
		};

		unsafe { msg_send![&*app, run] }
	}

	#[inline]
	fn exit(&self) -> Result<(), WResponse>
	{
		use objc2::{class, runtime::AnyObject};
		let raw: *mut NSApplication = unsafe {
			msg_send![class!(NSApplication), sharedApplication]
		};

		unsafe { let _: *mut AnyObject = msg_send![raw, retain]; }
		let Some(app) = (unsafe { Retained::from_raw(raw) }) else {
			 return Err(WResponse::UnexpectedError);
		};

		app.terminate(None);
		Ok(())
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
	fn new(window: Retained<NSWindow>) -> Option<Retained<Self>>
	{
		let mtm = MainThreadMarker::new()?;
		let this = Self::alloc(mtm).set_ivars(AppDelegateIvars { window });
		Some(unsafe { msg_send![super(this), init] })
	}
}
