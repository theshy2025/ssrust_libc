use crate::{linux::socket::Socket, log::buf_writer::LogBufWriter};

use super::traits::status::Status;

pub struct BaseLine {
    pub id:u64,
    pub socket:Socket,
    pub status:Status,
    pub log_buf_writer:LogBufWriter,
}

impl BaseLine {
    pub fn new(id:u64,socket:Socket,log_buf_writer:LogBufWriter) -> BaseLine {
        BaseLine { 
            id, 
            socket,
            log_buf_writer, 
            status: Status::Raw 
        }
    }
}