extern crate wayland_sys as wayland;
use std::ffi::CString;

fn main() {
    let display = unsafe { wayland::wl_display_create() };
    println!("created a display : {:?}", display);
}


