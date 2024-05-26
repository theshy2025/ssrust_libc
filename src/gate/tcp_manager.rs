use crate::{config, linux::{self}, log::log_trait::Log};

use super::Gate;

impl Gate {
    pub fn start_tcp_gate(&mut self) {
        match config::get("tcp_port") {
            Some(port) => {
                let port = port.parse().unwrap();
                self.tcp_gate = linux::socket::new_tcp_listener(port);
                self.epoll.register_read_event(self.tcp_gate.fd(), config::def::TCP_GATE_ID);
                self.log(format!("tcp gate socket listening on port {}",port));
            },

            None => {},
        }
    }
    
    pub fn accept_tcp_connect(&mut self) {
        let socket = self.tcp_gate.accept();
        let id = self.new_udp_2_vps_line();
        self.new_pc_line(socket);
    }
}

/* 

let mut buf = [0;256];
        let n = socket.recv(&mut buf);
        println!("{:?},{:?}",n,buf);


use std::net::TcpStream;

use crate::{config, log::Log};

use super::Gate;



    pub fn on_stream(&mut self,socket:TcpStream) {
        match config::loader::get("line_num") {
            Some(_) => {
                self.find_chick_for_pc(socket);
            },
            None => {
                //self.new_tcp_mainland_line(socket);
            },
        }
    }

    fn find_chick_for_pc(&mut self,socket:TcpStream) {
        let (tid,idle) = self.find_idle_udp_2_vps_line(true);
        self.log(format!("find_chick_for_pc tid:{},idle:{}",tid,idle));
        if tid > 0 {
            let id = self.new_pc_line(tid,socket);
            let line = self.lines.get_mut(&tid).unwrap();
            line.set_pair_id(id);
        } else {
            //self.err(format!("run out of chick"));
            panic!("run out of chick")
        }
    }
}
*/