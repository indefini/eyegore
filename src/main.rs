extern crate wayland_sys as wayland;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

/*
struct Signals
{
	create : *const wayland::wl_signal,
	activate : *const wayland::wl_signal,
	kill : *const wayland::wl_signal,
}

impl Signals {
	fn new() -> Signals
	{
		Signals 
	}
}
*/

fn main_wayland() {
    let display = unsafe { wayland::wl_display_create() };

    println!("created a display : {:?}", display);

    //let socket = unsafe { wayland::wl_display_add_socket(display, ptr::null()) };
    let socket_name = unsafe { wayland::wl_display_add_socket_auto(display)};

    let str = unsafe { CStr::from_ptr(socket_name).to_string_lossy().into_owned() };
    println!("created a socket : {}", str);

    // her setting WAYLAND_DISPLAY env variable
    let loopp = unsafe { wayland::wl_display_get_event_loop(display) };

    let signals = unsafe { wayland::create_signals() };

    unsafe { wayland::init_tmp(display) };

    //let client = unsafe { wayland::wl_client_create() };

    unsafe { wayland::wl_display_init_shm(display); }

    unsafe { wayland::wl_display_run(display); }
    unsafe { wayland::wl_display_destroy(display); }
}

fn main()
{	
    unsafe { wayland::main_wlc(); }
}


