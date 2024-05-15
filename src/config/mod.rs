use std::{collections::HashMap, sync::OnceLock};

use simple_config_parser::Config;

pub mod def;

pub static CONFIG_MAP:OnceLock<HashMap<String,String>> = OnceLock::new();


pub fn init(name:&String) {
    let path = format!("{}.config",name);
    let f = Config::new().file(&path).expect(&format!("{}",&path));
    let mut map = HashMap::new();
    map.insert("device".to_string(), name.to_string());
    for arr in f.data {
        map.insert(arr[0].clone(), arr[1].clone());
    }
    CONFIG_MAP.set(map).unwrap();
}

pub fn get(key:&str) -> Option<String> { 
    let map = CONFIG_MAP.get().unwrap();

    match map.get(key) {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}


/* 
pub mod loader;

pub const BUFF_SIZE:usize = 1024*8;
//pub const TCP_LIFE_TIME:i64 = 30;
pub const ATYP_HOST_NAME:u8 = 3;


pub const DNS_AGENT:u64 = 2;

pub const UDP_GATE_ID:u64 = 3;
//pub const TCP_ASSISTANT_FOR_UDP:u64 = 4;


use std::{collections::HashMap, sync::OnceLock};

use simple_config_parser::Config;






*/