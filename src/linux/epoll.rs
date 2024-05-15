use libc::epoll_event;

use crate::config::def::MAX_EVENT_NUM;

pub fn crate_epoll_instance() -> libc::c_int {
    unsafe { 
        libc::epoll_create1(0) 
    }
}

pub fn epoll_wait(fd:libc::c_int,ptr:*mut epoll_event,timeout:i32) -> libc::c_int {
    unsafe { 
        libc::epoll_wait(fd, ptr, MAX_EVENT_NUM as i32, timeout)
    }
}

pub fn epoll_ctl(epfd:libc::c_int,op:libc::c_int,fd:libc::c_int,events:u32,extra_id:u64) -> libc::c_int {
    unsafe { 
        let mut event = libc::epoll_event { events, u64: extra_id };
        let ptr: *mut libc::epoll_event = &mut event;
        libc::epoll_ctl(epfd, op, fd, ptr)
    }
}