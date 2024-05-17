use libc::c_int;

pub fn new() -> c_int {
    unsafe {
        libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0)
    }
}

pub fn bind(fd:c_int,port:u16) -> c_int {
    let sockaddr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port,
        sin_addr: libc::in_addr{ s_addr: libc::INADDR_ANY },
        sin_zero: [0; 8],
    };

    let len = std::mem::size_of::<libc::sockaddr_in>();
    
    unsafe {
        let ptr = &sockaddr as *const libc::sockaddr_in;
        libc::bind(fd, ptr as *const libc::sockaddr,len as libc::socklen_t)
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

/* 



pub fn connect(fd:c_int) -> c_int {
    unsafe {
        libc::connect(fd, std::ptr::null(), 0)
    }
}

unsafe {
            let sockaddr = libc::sockaddr_in {
                sin_family: libc::AF_INET as _,
                sin_port: port.to_be(),
                // 127.0.0.1
                sin_addr: libc::in_addr {
                    s_addr: 0x7f000001u32.to_be(),
                },
                sin_zero: [0; 8],libc::INADDR_ANY
            };
            let socklen = core::mem::size_of::<libc::sockaddr_in>();

            let sock = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if sock == -1 {
                return Err("could not create listen socket");
            }
*/