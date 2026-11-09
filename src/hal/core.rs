// src/hal/core.rs

mod mmio;
mod lpc111x;
mod lpc1115;

pub use self::mmio::*;
pub use self::lpc111x::*;
pub use self::lpc1115::*;