use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSRunningApplication, NSView,
    NSWindow, NSWindowStyleMask,
};
use cocoa::base::{nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};

use crate::WindowOpenOptions;

pub struct Window;

impl Window {
    pub fn open(options: WindowOpenOptions) -> Self {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            let rect = NSRect::new(
                NSPoint::new(0.0, 0.0),
                NSSize::new(options.width as f64, options.height as f64),
            );

            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    rect,
                    NSWindowStyleMask::NSTitledWindowMask,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease();
            window.center();
            window.setTitle_(NSString::alloc(nil).init_str(options.title));
            window.makeKeyAndOrderFront_(nil);

            let view = NSView::alloc(nil).init();
            window.setContentView_(view);

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            app.run();

            Window
        }
    }
}
