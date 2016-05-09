#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod rengine;


extern crate rand;

use rengine::ReDevice;

fn main() {
    let mut app = ReDevice::new();
    
    app.main_loop();
}