// src/hal/core/lpc1115.rs

#![cfg(feature = "lpc1115")]
#![allow(dead_code)]

pub const HAL_CORE_FLASH_ORIGIN: usize = 0x0000_0000;
pub const HAL_CORE_FLASH_SIZE: usize = 0x1_0000;
pub const HAL_CORE_RAM_ORIGIN: usize = 0x1000_0000;
pub const HAL_CORE_RAM_SIZE: usize = 0x2000;
pub const HAL_CORE_RAM_RESERVED: usize = 32;
pub const HAL_CORE_GPIO_PORT_COUNT: u32 = 4;
pub const HAL_CORE_ADC_CHANNEL_COUNT: u32 = 8;

pub fn hal_core_gpio_pin_count_get(port: u32) -> u32 {
    match port {
        0 => 12,
        1 => 12,
        2 => 12,
        3 => 6,
        _ => 0,
    }
}