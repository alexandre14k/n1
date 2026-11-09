// src/hal/driver.rs

mod clock;
mod gpio;
mod timer;

pub use self::clock::*;
pub use self::gpio::*;
pub use self::timer::*;
