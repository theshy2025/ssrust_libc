use std::io::Error;

use libc::epoll_event;

use crate::config::def::MAX_EVENT_NUM;

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
    pub fn wait(&self,timeout:i32) -> Vec<(u64,u32)> {
        let mut vec = Vec::new();
        let ee = libc::epoll_event{events:0,u64:0};
        let mut ees = [ee;MAX_EVENT_NUM];
        let ret = c::epoll::wait(self.fd, ees.as_mut_ptr(), timeout);
        
        if ret < 0 {
            panic!("epoll wait fail {}",Error::last_os_error())
        }

        for i in 0..ret as usize {
            let flags = ees[i].events;
            let id = ees[i].u64;
            vec.push((id,flags));
        }

        vec
    }
}
