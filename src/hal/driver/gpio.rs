// src/hal/driver/gpio.rs

use crate::hal::core::*;

pub const HAL_DRIVER_GPIO_PULL_NONE: u32 = 0;
pub const HAL_DRIVER_GPIO_PULL_DOWN: u32 = 1;
pub const HAL_DRIVER_GPIO_PULL_UP: u32 = 2;
pub const HAL_DRIVER_GPIO_PULL_REPEAT: u32 = 3;

const HAL_DRIVER_GPIO_FUNC_MASK: u32 = 0x7;
const HAL_DRIVER_GPIO_FUNC_PLAIN: u32 = 0;
const HAL_DRIVER_GPIO_FUNC_ALT: u32 = 1;
const HAL_DRIVER_GPIO_PULL_SHIFT: u32 = 3;
const HAL_DRIVER_GPIO_PULL_MASK: u32 = 0x3 << 3;
const HAL_DRIVER_GPIO_ADMODE: u32 = 1 << 7;
const HAL_DRIVER_GPIO_DATA_SHIFT: u32 = 2;

const HAL_DRIVER_GPIO_IOCON_P0: [u16; 12] = [
    0x00C, 0x010, 0x01C, 0x02C, 0x030, 0x034,
    0x04C, 0x050, 0x060, 0x064, 0x068, 0x074,
];

const HAL_DRIVER_GPIO_IOCON_P1: [u16; 12] = [
    0x078, 0x07C, 0x080, 0x090, 0x094, 0x0A0,
    0x0A4, 0x0A8, 0x014, 0x038, 0x06C, 0x098,
];

const HAL_DRIVER_GPIO_IOCON_P2: [u16; 12] = [
    0x008, 0x028, 0x05C, 0x08C, 0x040, 0x044,
    0x000, 0x020, 0x024, 0x054, 0x058, 0x070,
];

const HAL_DRIVER_GPIO_IOCON_P3: [u16; 6] = [
    0x084, 0x088, 0x09C, 0x0AC, 0x03C, 0x048,
];

pub fn hal_driver_gpio_pin_valid(port: u32, pin: u32) -> bool {
    pin < hal_core_gpio_pin_count_get(port)
}

fn hal_driver_gpio_iocon_offset_get(
    port: u32,
    pin: u32,
) -> Option<usize> {
    let table: &[u16] = match port {
        0 => &HAL_DRIVER_GPIO_IOCON_P0,
        1 => &HAL_DRIVER_GPIO_IOCON_P1,
        2 => &HAL_DRIVER_GPIO_IOCON_P2,
        3 => &HAL_DRIVER_GPIO_IOCON_P3,
        _ => return None,
    };
    table.get(pin as usize).map(|v| *v as usize)
}

fn hal_driver_gpio_iocon_addr_get(
    port: u32,
    pin: u32,
) -> Option<usize> {
    if !hal_driver_gpio_pin_valid(port, pin) {
        return None;
    }
    hal_driver_gpio_iocon_offset_get(port, pin)
        .map(|off| HAL_CORE_IOCON_BASE + off)
}

fn hal_driver_gpio_function_plain_get(port: u32, pin: u32) -> u32 {
    match (port, pin) {
        (0, 0) | (0, 10) | (0, 11) => HAL_DRIVER_GPIO_FUNC_ALT,
        (1, 0) | (1, 1) | (1, 2) | (1, 3) => {
            HAL_DRIVER_GPIO_FUNC_ALT
        }
        _ => HAL_DRIVER_GPIO_FUNC_PLAIN,
    }
}

fn hal_driver_gpio_data_addr_get(port: u32, pin: u32) -> usize {
    hal_core_gpio_base_get(port)
        + ((1usize << pin) << HAL_DRIVER_GPIO_DATA_SHIFT)
}

fn hal_driver_gpio_dir_addr_get(port: u32) -> usize {
    hal_core_gpio_base_get(port) + HAL_CORE_GPIO_DIR
}

pub fn hal_driver_gpio_function_set(
    port: u32,
    pin: u32,
    func: u32,
) -> bool {
    match hal_driver_gpio_iocon_addr_get(port, pin) {
        Some(addr) => {
            hal_core_mmio_modify(
                addr,
                HAL_DRIVER_GPIO_FUNC_MASK,
                func,
            );
            true
        }
        None => false,
    }
}

pub fn hal_driver_gpio_select(port: u32, pin: u32) -> bool {
    let func = hal_driver_gpio_function_plain_get(port, pin);
    hal_driver_gpio_function_set(port, pin, func)
}

pub fn hal_driver_gpio_pull_set(
    port: u32,
    pin: u32,
    mode: u32,
) -> bool {
    match hal_driver_gpio_iocon_addr_get(port, pin) {
        Some(addr) => {
            hal_core_mmio_modify(
                addr,
                HAL_DRIVER_GPIO_PULL_MASK,
                mode << HAL_DRIVER_GPIO_PULL_SHIFT,
            );
            true
        }
        None => false,
    }
}

pub fn hal_driver_gpio_analog_set(
    port: u32,
    pin: u32,
    analog: bool,
) -> bool {
    match hal_driver_gpio_iocon_addr_get(port, pin) {
        Some(addr) => {
            if analog {
                hal_core_mmio_clear(addr, HAL_DRIVER_GPIO_ADMODE);
            } else {
                hal_core_mmio_set(addr, HAL_DRIVER_GPIO_ADMODE);
            }
            true
        }
        None => false,
    }
}

pub fn hal_driver_gpio_write(port: u32, pin: u32, high: bool) {
    if !hal_driver_gpio_pin_valid(port, pin) {
        return;
    }
    let value = if high { 1u32 << pin } else { 0 };
    hal_core_mmio_write(
        hal_driver_gpio_data_addr_get(port, pin),
        value,
    );
}

pub fn hal_driver_gpio_set(port: u32, pin: u32) {
    hal_driver_gpio_write(port, pin, true);
}

pub fn hal_driver_gpio_clear(port: u32, pin: u32) {
    hal_driver_gpio_write(port, pin, false);
}

pub fn hal_driver_gpio_read(port: u32, pin: u32) -> bool {
    if !hal_driver_gpio_pin_valid(port, pin) {
        return false;
    }
    let value = hal_core_mmio_read(
        hal_driver_gpio_data_addr_get(port, pin),
    );
    value & (1u32 << pin) != 0
}

pub fn hal_driver_gpio_toggle(port: u32, pin: u32) {
    let level = hal_driver_gpio_read(port, pin);
    hal_driver_gpio_write(port, pin, !level);
}

pub fn hal_driver_gpio_input_set(port: u32, pin: u32) {
    if hal_driver_gpio_pin_valid(port, pin) {
        hal_core_mmio_clear(
            hal_driver_gpio_dir_addr_get(port),
            1u32 << pin,
        );
    }
}

pub fn hal_driver_gpio_output_set(
    port: u32,
    pin: u32,
    high: bool,
) {
    if hal_driver_gpio_pin_valid(port, pin) {
        hal_driver_gpio_write(port, pin, high);
        hal_core_mmio_set(
            hal_driver_gpio_dir_addr_get(port),
            1u32 << pin,
        );
    }
}