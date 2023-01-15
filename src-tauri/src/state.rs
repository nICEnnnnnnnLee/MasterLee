
lazy_static::lazy_static! {
    pub static ref PROXY_ENABLED: bool= false;
}


pub fn enable_proxy(enable: bool) {
    let p = std::ptr::addr_of!(*PROXY_ENABLED) as *mut bool;
    unsafe {
        std::ptr::write(p, enable);
    }
}

pub fn proxy_enabled() -> bool {
    *PROXY_ENABLED
}
