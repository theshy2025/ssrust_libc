use super::buf_writer::LogBufWriter;

pub trait Log {
    fn id(&self) -> u64 {0}

    fn logger(&mut self) -> &mut LogBufWriter;

    fn log(&mut self,s:String) {
        self.logger().add(s);
        self.logger().flush();
    }

    fn err(&mut self,s:String) {
        crate::log::err(format!("[{}]{}",self.id(),s));
        self.log(s);
    }
}