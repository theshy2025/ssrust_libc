use libc::{c_int,sockaddr_in, ssize_t};

pub fn new(ty:c_int) -> c_int {
    unsafe {
        libc::socket(libc::AF_INET, ty, 0)
    }
}

pub fn bind(fd:c_int,port:u16) -> c_int {
    let addr = new_sockaddr_in(port.to_be());
    let len = std::mem::size_of::<sockaddr_in>();
    
    unsafe {
        libc::bind(fd, &addr as *const _ as _,len as _)
    }
}

pub fn listen(fd:c_int,backlog:c_int) -> c_int {
    unsafe {
        libc::listen(fd, backlog)
    }
}

pub fn accept(fd:c_int) -> c_int {
    unsafe {
        libc::accept(fd, std::ptr::null_mut(), std::ptr::null_mut())
    }
}

pub fn recv(fd:c_int,buf: &mut [u8]) -> ssize_t {
    unsafe {
        libc::recv(fd,buf.as_mut_ptr() as _,buf.len(),0)
    }
}

pub fn send(fd:c_int,buf:&[u8]) -> ssize_t {
    unsafe {
        libc::send(fd,buf.as_ptr() as _,buf.len(),0)
    }
}

pub fn send_to(fd:c_int,buf: &[u8],ip:&str,port:u16) -> ssize_t {
    let mut addr = new_sockaddr_in(port.to_be());
    let ip:std::net::Ipv4Addr = ip.parse().unwrap();
    println!("{}",ip);
    addr.sin_addr.s_addr = u32::from(ip).to_be();
    let len = std::mem::size_of::<sockaddr_in>();
    unsafe {
        libc::sendto(fd, buf.as_ptr() as _, buf.len(), 0, &addr as *const _ as _, len as _)
    }
}

/* 

 unsafe {
            let mut dest_addr : libc::sockaddr_in = std::mem::zeroed();
            let len = std::mem::size_of_val(&dest_addr) as u32;

            dest_addr.sin_addr.s_addr = inet_addr(ipaddr.as_ptr() as *const libc::c_char);
            dest_addr.sin_port = port.to_be() as u16;
            dest_addr.sin_family = libc::AF_INET as u16;

            ret = libc::sendto(self.fd,
                               buf as *mut [u8] as *mut libc::c_void,
                               buf_len,
                               0,
                               &dest_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                               len);
        }

*/

pub fn setsockopt<T:Copy>(fd:c_int,level:c_int,name:c_int,value:T) -> c_int {
    let ptr = &value as *const T as *const libc::c_void;
    unsafe {
        libc::setsockopt(fd, level, name, ptr, std::mem::size_of::<T>() as libc::socklen_t)
    }
}

pub fn getsockopt<T>(fd:c_int,level:c_int,optname:c_int,value:&mut T) -> c_int {
    let ptr = value as *mut T as *mut libc::c_void;
    let mut len = std::mem::size_of::<T>() as libc::socklen_t;
    unsafe {
        libc::getsockopt(fd, level, optname, ptr, &mut len)
    }
}

pub fn set_non_block(fd:c_int) -> c_int {
    unsafe {
        libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK)
    }
}

pub fn get_sock_name(fd:c_int,addr:&mut sockaddr_in) -> c_int {
    unsafe {
        let mut len = std::mem::size_of_val(addr) as libc::c_uint;
        libc::getsockname(fd, addr as *mut _ as _, &mut len)
    }
}

pub fn new_sockaddr_in(port:u16) -> sockaddr_in {
    sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: port,
        sin_addr: libc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    }
}

/* 


let is_valid = unsafe {
        let mut ty: libc::c_int = mem::zeroed();
        let mut ty_len = mem::size_of_val(&ty) as libc::c_uint;
        let mut sockaddr: libc::sockaddr = mem::zeroed();
        let mut sockaddr_len = mem::size_of_val(&sockaddr) as libc::c_uint;
        libc::getsockname(fd, &mut sockaddr, &mut sockaddr_len) == 0
            && libc::getsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_TYPE,
                mem::transmute(&mut ty),
                &mut ty_len,
            ) == 0

fn get_opt<T>(
    sock: c_int,
    opt: c_int,
    val: c_int,
    payload: &mut T,
    size: &mut socklen_t,
) -> io::Result<()> {
    unsafe {
        let payload = payload as *mut T as *mut c_void;
        cvt_linux_error(libc::getsockopt(sock, opt, val, payload as *mut _, size))?;
        Ok(())
    }
}

pub fn connect(fd:c_int) -> c_int {
    unsafe {
        libc::connect(fd, std::ptr::null(), 0)
    }
}


*/