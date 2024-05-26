use std::io::Error;

use super::c::{self, socket::new_sockaddr_in};

#[derive(Debug)]
pub struct Socket {
    fd:i32
}

impl Socket {
    pub fn invalid() -> Socket {
        Socket { fd:0 }
    }

    pub fn new_tcp_socket() -> Socket {
        let fd = c::socket::new(libc::SOCK_STREAM);
        if fd < 0 {
            panic!("create linux tcp socket fail {}",Error::last_os_error())
        }
        Socket { fd }
    }

    pub fn new_udp_socket() -> Socket {
        let fd = c::socket::new(libc::SOCK_DGRAM);
        if fd < 0 {
            panic!("create linux udp socket fail {}",Error::last_os_error())
        }
        Socket { fd }
    }
}

impl Socket {
    pub fn fd(&self) -> i32 {
        self.fd
    }

    pub fn local_addr(&self) {
        let mut addr = c::socket::new_sockaddr_in(0);
        let ret = c::socket::get_sock_name(self.fd, &mut addr);
        let ip = addr.sin_addr.s_addr;
        println!("{},{},{}",ret,ip,addr.sin_port.to_be());
    }
}

impl Socket {
    pub fn bind(&self,port:u16) {
        let ret = c::socket::bind(self.fd,port);
        if ret < 0 {
            panic!("socket fd:{} bind to port:{} fail {}",self.fd,port,Error::last_os_error())
        }
    }

    pub fn listen(&self) {
        let ret = c::socket::listen(self.fd, 128);
        if ret < 0 {
            panic!("socket fd:{} listen fail {}",self.fd,Error::last_os_error())
        }
    }

    pub fn accept(&self) -> Socket {
        let fd = c::socket::accept(self.fd);
        if fd < 0 {
            panic!("socket fd:{} accept fail {}",self.fd,Error::last_os_error())
        }

        Socket{ fd }
    }

}

impl Socket {
    pub fn recv(&self,buf: &mut [u8]) -> std::io::Result<usize> {
        let ret = c::socket::recv(self.fd,buf);
        if ret == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ret as usize)
    }

    pub fn send_to(&self,buf: &[u8],ip:&str,port:u16) -> std::io::Result<usize> {
        let ret = c::socket::send_to(self.fd,buf,ip,port);
        if ret == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ret as usize)
    }
}

impl Socket {
    pub fn set_non_block(&self) {
        let ret = c::socket::set_non_block(self.fd);
        if ret == -1 {
            panic!("socket fd:{} set_non_block fail {}",self.fd,Error::last_os_error())
        }
    }
}

pub fn new_tcp_listener(port:u16) -> Socket {
    let sock = Socket::new_tcp_socket();
    sock.bind(port);
    sock.listen();
    sock
}

pub fn random_port_open() -> Socket {
    let sock = Socket::new_udp_socket();
    sock.bind(0);
    let mut addr = new_sockaddr_in(0);
    c::socket::get_sock_name(sock.fd(), &mut addr);
    println!("{}",addr.sin_port.to_be());
    
    sock
}