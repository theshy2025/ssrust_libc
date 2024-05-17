use crate::config::def;

use super::Gate;


impl Gate {
    pub fn on_epoll_event(&mut self,id:u64,flags:i32) {
        println!("on_epoll_event id:{},flags:{}",id,flags);
        if flags & libc::EPOLLIN > 0 {
            self.epoll_in(id);
        }

        
    }
}

impl Gate {
    pub fn epoll_in(&mut self,id:u64) {
        println!("epoll_in id:{}",id);
        match id {
            def::TCP_GATE_ID => self.accept_tcp_connect(),
            //def::UDP_GATE_ID => self.accept_udp_connect(),
            other => todo!(),//self.on_read_able_event(other),
        }
    }
}



/*

pub fn on_epoll_event(&mut self,id:u64,evt:EpollEvent) {
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

    
use nix::sys::epoll::EpollFlags;

use crate::{config::{BUFF_SIZE, TCP_GATE_ID,UDP_GATE_ID}, log::Log};

use super::Gate;

impl Gate {
    pub fn epoll_in(&mut self,id:u64) {
        match id {
            TCP_GATE_ID => self.accept_tcp_connect(),
            UDP_GATE_ID => self.accept_udp_connect(),
            other => self.on_read_able_event(other),
        }
    }

    pub fn on_write_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_write_able();
    }

    pub fn on_read_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        let mut buf = [0;BUFF_SIZE];
        let (start,stop,data_tag) = line.on_read_able(&mut buf);
        if stop > 0 {
            let pid = line.pair_id();
            if pid > 0 {
                let line = self.lines.get_mut(&pid).unwrap();
                line.on_pair_data(&buf[start..stop],data_tag);
            }
        }
    }

    pub fn on_rd_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_rd_hang_up();
    }

    pub fn epoll_err(&mut self,id:u64) {
        match id {
            TCP_GATE_ID => self.log(format!("gate error")),
            other => {
                let line = self.lines.get_mut(&other).unwrap();
                line.on_error();
            }
        }
    }

    pub fn on_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_hang_up();
    }

    pub fn on_other_event(&mut self,id:u64,flags:EpollFlags) {
        self.log(format!("[{}]on_other_event {:?}",id,flags));
    }
}

*/