use crate::{config::{self, def::TCP_GATE_ID}, linux::{self, epoll::Epoll}, log::{buf_writer::LogBufWriter, log_trait::Log}};

mod epoll;
mod event;

mod tcp_manager;

pub struct Gate {
    epoll:Epoll,
    tcp_gate_fd:i32,
    //lines:HashMap<u64,Box<dyn Line>>,
    buf_writer:LogBufWriter,
    udp_gate_fd:i32,
}

impl Gate {
    pub fn new() -> Gate {
        let dir = crate::log::device_log_path();
        let path = format!("{}/{}.log",dir,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        let epoll = Epoll::new();
        Gate {
            epoll,
            tcp_gate_fd: 0,
            udp_gate_fd: 0 ,
            buf_writer,
        }
    }

    
}


impl Gate {
    pub fn start(&mut self) {
        self.init();
        loop {
            crate::global::next_frame();
            //self.tick();
            self.poll(0);
            //self.check_dns_result();
            //self.gather_dns_query();
            //self.gather_client_hello();
        }
    }
}

impl Gate {
    fn init(&mut self) {
        self.start_tcp_gate();
        match config::get("line_num") {
            Some(n) => {
                let _n:u8 = n.parse().unwrap();
                
                //self.create_udp_2_vps_lines(n);
            }

            None => {
                //self.activate_dns_manager();
                //self.start_udp_gate();
            },
        }
    }

    fn start_tcp_gate(&mut self) {
        let port = config::get("tcp_port").unwrap();
        self.tcp_gate_fd = linux::socket::new_tcp_listener(port.parse().unwrap());
        self.epoll.register_read_event(self.tcp_gate_fd, TCP_GATE_ID);
        /*
        let tcp_gate = TcpListener::bind(format!("0.0.0.0:{}",port)).unwrap();
        tcp_gate.set_nonblocking(true).unwrap();
        let tcp_gate_fd = tcp_gate.as_raw_fd();
        
        self.log(format!("tcp gate socket listening on {:?}",tcp_gate.local_addr()));
        self.tcp_gate = Some(tcp_gate);*/
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
mod line_creater;
mod line_manager;
mod udp_manager;

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