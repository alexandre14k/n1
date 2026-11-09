// src/main.rs

#![allow(dead_code)]
#![allow(unused_imports)]

#![no_std]
#![no_main]

mod app;
mod hal;

use app::*;
use hal::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    main_init();
    main_loop()
}

fn main_init() {
    hal_boot_init();
    app_statemachine_init();
}

fn main_loop() -> ! {
    loop {
        app_statemachine_step();
    }
}