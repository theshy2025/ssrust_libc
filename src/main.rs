use std::env;

use gate::Gate;

mod linux;
mod global;
mod config;
mod log;
mod gate;
mod line;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need to be [cargo run local] or [cargo run server]");
    }
    config::init(&args[1]);
    log::init();
    Gate::new().start();
}

/* 

[dependencies]
chrono = "0.4.37"

simple-dns = "0.7.0"
nix = { version = "0.28.0", features = ["poll","event"] }
socket2 = "0.5.6"

*/