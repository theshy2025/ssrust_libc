use std::io::Error;

use super::c;

pub fn new_tcp_listener(port:u16) -> i32 {
    let fd = new_tcp_socket();
    bind_socket(fd,port.to_be());
    c::socket::listen(fd, 128);
    fd
}

pub fn new_tcp_socket() -> i32 {
    let fd = c::socket::new();
    if fd < 0 {
        panic!("create tcp socket fail {}",Error::last_os_error())
    }
    fd
}

pub fn bind_socket(fd:i32,port:u16) {
    let ret = c::socket::bind(fd,port);
    if ret < 0 {
        panic!("bind socket fail fd:{},port:{},{}",fd,port,Error::last_os_error())
    }
}

pub fn accept_tcp(fd:i32) -> i32 {
    let ret = c::socket::accept(fd);
    if ret < 0 {
        panic!("accept tcp socket fail fd:{},{}",fd,Error::last_os_error())
    }
    ret
}