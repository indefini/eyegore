extern crate wayland_sys as wayland;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

fn main() {
    let display = unsafe { wayland::wl_display_create() };

    println!("created a display : {:?}", display);

    //let socket = unsafe { wayland::wl_display_add_socket(display, ptr::null()) };
    let socket_name = unsafe { wayland::wl_display_add_socket_auto(display)};

    let str = unsafe { CStr::from_ptr(socket_name).to_string_lossy().into_owned() };
    println!("created a socket : {}", str);

    //let client = unsafe { wayland::wl_client_create() };

    unsafe { wayland::wl_display_run(display); }
    unsafe { wayland::wl_display_destroy(display); }
}


