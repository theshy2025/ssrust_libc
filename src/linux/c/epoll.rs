use libc::{c_int, epoll_event};

use crate::config::def::MAX_EVENT_NUM;

pub fn new() -> c_int {
    unsafe { 
        libc::epoll_create1(0) 
    }
}

pub fn ctl(epfd:c_int,op:c_int,fd:c_int,events:u32,extra_id:u64) -> c_int {
    unsafe { 
        let mut event = epoll_event { events, u64: extra_id };
        libc::epoll_ctl(epfd, op, fd, &mut event)
    }
}

pub fn wait(epfd:c_int,ptr:*mut epoll_event,timeout:i32) -> c_int {
    unsafe { 
        libc::epoll_wait(epfd, ptr, MAX_EVENT_NUM as i32, timeout)
    }
}