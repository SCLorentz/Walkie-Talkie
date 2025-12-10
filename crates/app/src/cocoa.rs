/*use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, sel, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
	NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu, NSMenuItem,
	NSWindow, NSWindowController,
};
use objc2_foundation::{ns_string, NSNotification, NSObject, NSObjectProtocol};*/

use crate::CfgDecoration;

#[derive(Clone)]
pub struct CocoaWinDecoration {
	default: CfgDecoration,
}
