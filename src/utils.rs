use x11::xlib;
use std::ptr;

pub fn query_pointer() -> (i32, i32) {
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        panic!("Unable to open X display");
    }

    let root = unsafe { xlib::XDefaultRootWindow(display) };
    let mut root_return = 0;
    let mut child_return = 0;
    let mut root_x = 0;
    let mut root_y = 0;
    let mut win_x = 0;
    let mut win_y = 0;
    let mut mask_return = 0;

    let status = unsafe {
        xlib::XQueryPointer(
            display,
            root,
            &mut root_return,
            &mut child_return,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask_return,
        )
    };

    unsafe {
        xlib::XCloseDisplay(display);
    }

    if status == 0 {
        panic!("Unable to query pointer");
    }

    (root_x, root_y)
}

pub fn get_screen_size() -> (i32, i32) {
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        panic!("Unable to open X display");
    }

    let screen = unsafe { xlib::XDefaultScreen(display) };
    let width = unsafe { xlib::XDisplayWidth(display, screen) };
    let height = unsafe { xlib::XDisplayHeight(display, screen) };

    unsafe {
        xlib::XCloseDisplay(display);
    }

    (width, height)
}
