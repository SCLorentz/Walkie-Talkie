/*use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, sel, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu, NSMenuItem,
	NSWindow, NSWindowController,
};
use objc2_foundation::{ns_string, NSNotification, NSObject, NSObjectProtocol};*/

#[cfg(target_os = "macos")]
use objc2_app_kit::NSView;

#[cfg(target_os = "macos")]
use objc2::rc::Retained;

use crate::DecorationMode;

pub struct CocoaWinDecoration {
	pub mode: DecorationMode,
	pub view: Retained<NSView>,
}
