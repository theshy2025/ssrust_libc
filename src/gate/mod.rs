use crate::{linux::{epoll::Epoll, socket::Socket}, log::{buf_writer::LogBufWriter, log_trait::Log}};

mod epoll;
mod event;

mod line_creater;
mod tcp_manager;
mod udp_manager;

pub struct Gate {
    epoll:Epoll,
    tcp_gate:Socket,
    udp_gate:Socket,
    //lines:HashMap<u64,Box<dyn Line>>,
    buf_writer:LogBufWriter,
}

impl Gate {
    pub fn new() -> Gate {
        let dir = crate::log::device_log_path();
        let path = format!("{}/{}.log",dir,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        Gate {
            epoll:Epoll::new(),
            tcp_gate: Socket::invalid(),
            udp_gate: Socket::invalid() ,
            buf_writer,
        }
    }
}


impl Gate {
    pub fn start(&mut self) {
        
        self.start_tcp_gate();
        
        self.open_udp_port();

        //self.activate_dns_manager();

        loop {
            crate::global::next_frame();
            //self.tick();
            self.poll();
            //self.check_dns_result();
            //self.gather_dns_query();
            //self.gather_client_hello();
        }
    }
}

impl Log for Gate {
    fn logger(&mut self) -> &mut LogBufWriter {
       &mut self.buf_writer
    }
}

/* 

use std::{collections::HashMap, net::{TcpListener, UdpSocket}, os::fd::AsFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags};

use crate::{config::{self, TCP_GATE_ID}, global, line::traits::Line, log::{buf_writer::LogBufWriter, Log}};


mod event;

mod line_manager;


mod dns_manager;


pub struct Gate {
    tcp_gate:TcpListener,
    epoll:Epoll,
    lines:HashMap<u64,Box<dyn Line>>,
    buf_writer:LogBufWriter,
    udp_gate:Option<UdpSocket>,
}

impl Gate {
    pub fn new() -> Gate {
        let port = config::loader::get("tcp_port").unwrap();
        let tcp_gate = TcpListener::bind(format!("0.0.0.0:{}",port)).unwrap();
        
        let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
        let dir = crate::log::device_log_path();
        let path = format!("{}/{}.log",dir,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        Gate{ tcp_gate , epoll , lines:HashMap::new(), buf_writer, udp_gate: None }
    }
}

impl Gate {
    pub fn start(&mut self) {
        self.init();
        loop {
            global::next_frame();
            
            
            self.tick();

            self.poll();
            

            self.check_dns_result();
            self.gather_dns_query();
            self.gather_client_hello();
            
        }
    }

    

    
}




*/