use std::io::Error;

use super::c;

pub fn new_tcp_listener() -> i32 {
    let fd = new_tcp_socket();
    c::socket::bind();
    fd
}

pub fn new_tcp_socket() -> i32 {
    let fd = c::socket::new();
    if fd < 0 {
        panic!("create tcp socket fail {}",Error::last_os_error())
    }
    fd
}