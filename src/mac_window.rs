use core_graphics::base::CGFloat;
use objc::{class, msg_send, sel, sel_impl};
use core_graphics::geometry::CGRect;
use objc::runtime::{Class, Object};
use crate::window_config::WindowConfig;

// needs to be initialized for the objc macros to work
extern crate cocoa;

pub const NS_WINDOW_TITLED: u32 = 1;
pub const NS_WINDOW_CLOSABLE: u32 = 2;
pub const NS_WINDOW_MINIATURIZABLE: u32 = 4;
pub const NS_WINDOW_RESIZABLE: u32 = 8;

//
pub const NS_BACKING_STORE_BUFFERED: u8 = 2;

pub fn create_window (config: &WindowConfig) -> u8 {


    // using unsafe, because we are calling Objective-C and C functions
    unsafe {
        // fetch the NSApplication class, which is the 'heart' of the application on macOS
        // it is the class that is responsible for the application's main window and menu bar
        // there is only one of these per application and manages the whole application's life cycle
        let ns_app = Class::get("NSApplication").unwrap();

        // msg_send! is a macro that calls a method on an Objective-C object
        // it takes the object, the method name, and the arguments to the method
        // the return value is the result of the method call, kinda like a return value
        // it always returns a pointer to the object, so we need to cast it to a *mut Object
        let app: *mut Object = msg_send![ns_app, sharedApplication];

        // fetch the NSWindow class, which is the class that represents a window
        let ns_window = Class::get("NSWindow").unwrap();

        // this line creates a new window with the given size and positions
        // it uses the CGRect struct to represent a rectangle, which can be used to create a window
        let rect = CGRect::new(
            &core_graphics::geometry::CGPoint::new(config.position.0 as CGFloat, config.position.1 as CGFloat),
            &core_graphics::geometry::CGSize::new(config.width as CGFloat, config.height as CGFloat));


        // create a variable with the right window properties
        // the value of the variable is based on the mod from macOS, default is ob1111
        let style_mask =
            if config.has_title { NS_WINDOW_TITLED } else { 0 } |
                if config.closable { NS_WINDOW_CLOSABLE } else { 0 } |
                if config.resizable { NS_WINDOW_RESIZABLE } else { 0 } |
                if config.minimizable { NS_WINDOW_MINIATURIZABLE } else { 0 };


        // set the window's backing store / the way the window is rendered
        let backing = config.backing;

        // allocate the window as a call to the alloc objc method from the NSWindow class
        let window: *mut Object = msg_send![ns_window, alloc];

        // initialize the window with the given properties
        // this is the same as the following code:
        // [[NSWindow alloc] initWithContentRect:... styleMask:... backing:... defer:NO];
        let window: *mut Object = msg_send![
            window,
            initWithContentRect:rect
            styleMask:style_mask
            backing:backing
            defer:false];

        // create the window's title using by converting the string to a C string
        let title = std::ffi::CString::new(config.title.clone()).unwrap();

        // convert the C string to an NSString object, which is a subclass of NSString
        // this is done by passing the previous CString in.
        let ns_string: *mut Object = msg_send![
            Class::get("NSString").unwrap(),
            stringWithUTF8String:title.as_ptr()
        ];

        // Set the title of the window,
        // this is done by calling the setTitle method on the window object
        // Ignore the return value, we don't care about it
        let _: () = msg_send![window, setTitle:ns_string];

        // Put the window in front of all other windows
        // and again ignore the return value
        // Same as the following obj-c code:
        // [window makeKeyAndOrderFront:nil]
        let _: () = msg_send![window, makeKeyAndOrderFront: std::ptr::null::<Object>()];

        let _: () = msg_send![app, run];

    }

    0
}