
pub mod status;
/*use std::any::Any;

use self::{dns::LineTraitDns, event::LineTraitEvent, heart_beat::LineTraitHeartBeat, tunnel_response::LineTraitTunnelResponse, pair::LineTraitPair, status::Status, tunnel::LineTraitTunnel};

pub mod heart_beat;
pub mod network;

pub mod pair;
pub mod event;
pub mod tunnel;
pub mod tunnel_response;
pub mod dns;

pub trait Line : 
LineTraitHeartBeat
+LineTraitEvent
+LineTraitPair
+LineTraitTunnel
+LineTraitTunnelResponse
+LineTraitDns
{
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn tick(&mut self) {
        //let clock = Instant::now();
        self.next_status();

        match self.status() {
            Status::Establish => self.send_heart_beat(),
            _ => {},
        }
        
        self.resend_timeout_packet();
        
        self.check_tunnel_response_packet()
        //let n = clock.elapsed().as_micros();
        //if n > 1 {
            //self.log(format!("line_tick:{}",n));
        //}
    }

}*/