extern crate wayland_sys as wayland;
use std::ffi::CString;

fn main() {
    let display = unsafe { wayland::wl_display_create() };

    println!("created a display : {:?}", display);

    //let client = unsafe { wayland::wl_client_create() };

    unsafe { wayland::wl_display_destroy(display); }
}


