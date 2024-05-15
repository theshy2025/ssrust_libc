use std::os::fd::AsRawFd;

use crate::{config::def::MAX_EVENT_NUM, linux};

use super::Gate;



impl Gate {
    pub fn epoll_wait(&mut self,timeout:i32) {
        let event = libc::epoll_event{events:0,u64:0};
        let mut events = [event;MAX_EVENT_NUM as usize];
        let ret = linux::epoll::epoll_wait(self.epoll_fd,events.as_mut_ptr(),timeout);
        assert!(ret >= 0);
        if ret > 0 {
            for i in 0..ret as usize {
                let a = events[i].events;
                let b = events[i].u64;
                let stream = self.tcp_gate.as_ref().unwrap().accept();
                println!("{},{},{},{},{:?}",i,a,b,ret,stream);
                let (stream,_) = stream.unwrap();
                self.register_read_event(stream.as_raw_fd(), 88);
            }

            let ret = linux::epoll::epoll_wait(self.epoll_fd,events.as_mut_ptr(),timeout);
            println!("{}",ret);
        }

        
    }
}

impl Gate {
    pub fn register_read_event(&self,fd:i32,id:u64) {
        let flag = libc::EPOLLIN | libc::EPOLLRDHUP;
        self.add_fd(fd, id,  flag as u32);
    }
}

impl Gate {
    pub fn add_fd(&self,fd:i32,id:u64,events:u32) {
        self.ctl_fd(fd,libc::EPOLL_CTL_ADD,events,id);
    }

    pub fn remove_fd(&self,fd:i32) {
        self.ctl_fd(fd,libc::EPOLL_CTL_DEL,0,0);
    }

    pub fn ctl_fd(&self,fd:i32,op:i32,events:u32,extra_id:u64) {
        let ret = linux::epoll::epoll_ctl(self.epoll_fd,op,fd,events,extra_id);
        if ret < 0 {
            let msg = std::io::Error::last_os_error();
            panic!("{:?}",msg);
        }
    }
}


/* 

use std::time::Instant;

use super::Gate;




use std::{os::fd::BorrowedFd, time::Instant};

use nix::sys::epoll::{EpollEvent,EpollFlags,EpollTimeout};

use crate::log::{self};

use super::Gate;

impl Gate {
    pub fn poll(&mut self) {
        let t = Instant::now();
        let raw = EpollEvent::empty();
        let mut events = [raw;32];
        let timeout = EpollTimeout::from(3u16);
        self.epoll.wait(&mut events, timeout).unwrap();

        

        for v in events {
            let id = v.data();
            if id > 0 {
                self.on_epoll_event(id,v);
            }
        }
        
    }

    fn on_epoll_event(&mut self,id:u64,evt:EpollEvent) {
        for flags in evt.events() {
            //self.log(format!("id:{},flags:{:?}",id,flags));
            match flags {
                EpollFlags::EPOLLIN => self.epoll_in(id),
                EpollFlags::EPOLLOUT => self.on_write_able_event(id),
                EpollFlags::EPOLLRDHUP => self.on_rd_hang_up_event(id),
                EpollFlags::EPOLLERR => self.epoll_err(id),
                EpollFlags::EPOLLHUP => self.on_hang_up_event(id),
                other => self.on_other_event(id,other),
            }
        }
    }
    

    pub fn register_read_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLIN);
        flags.insert(EpollFlags::EPOLLRDHUP);
        self.add_fd(fd, id, flags);
    }

    pub fn register_write_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLOUT);
        self.add_fd(fd, id, flags);
    }

    pub fn add_fd(&self,fd:BorrowedFd<'_>,id:u64,flags:EpollFlags) {
        let event = EpollEvent::new(flags,id);
        
        match self.epoll.add(fd, event) {
            Ok(_) => {
                //let str = flags_str_name(flags);
                //log::err(format!("id:{} add_fd {:?} success",id,fd));
            },

            Err(e) => log::err(format!("[{}]add_fd {:?} fail {}",id,fd,e)),
        }
    }

    pub fn remove_fd(&self,fd:BorrowedFd<'_>) {
        match self.epoll.delete(fd) {
            Ok(_) => {
                //log::err(format!("remove_fd {:?} success",fd));
            },
            
            Err(e) => log::err(format!("remove_fd {:?} fail {}",fd,e)),
        }
    }
}

*/