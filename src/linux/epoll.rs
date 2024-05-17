use std::io::Error;

use libc::epoll_event;

use super::c;

pub struct Epoll {
    fd:i32
}

impl Epoll {
    pub fn new() -> Epoll {
        let fd = c::epoll::new();
        if fd < 0 {
            panic!("create epoll instance fail {}",Error::last_os_error())
        }
        Epoll { fd }
    }
}

impl Epoll {
    pub fn register_read_event(&self,fd:i32,id:u64) {
        let flag = libc::EPOLLIN | libc::EPOLLRDHUP;
        let ret = c::epoll::ctl(self.fd, libc::EPOLL_CTL_ADD, fd, flag as u32, id);
        if ret < 0 {
            panic!("register_read_event fail {}",Error::last_os_error())
        }
    }
}

impl Epoll {
    pub fn wait(&self,ptr:*mut epoll_event,timeout:i32) -> usize {
        let ret = c::epoll::wait(self.fd, ptr, timeout);
        if ret < 0 {
            panic!("epoll wait fail {}",Error::last_os_error())
        }
        ret as usize
    }
}
