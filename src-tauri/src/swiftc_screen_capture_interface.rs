extern "C" {
    //fn hello_world() -> bool;
    fn request_screen_capture_access() -> bool;
}

pub fn check_access() -> bool {
    unsafe { request_screen_capture_access() }
}
