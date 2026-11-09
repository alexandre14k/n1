// src/app/pinout.rs

mod om13087;

pub use self::om13087::*;

use crate::hal::*;

pub const APP_PINOUT_PORT_CODE_BASE: u32 = 0x0A;

pub const APP_PINOUT_LED_RED: u32 = 0;
pub const APP_PINOUT_LED_GREEN: u32 = 1;
pub const APP_PINOUT_LED_BLUE: u32 = 2;
pub const APP_PINOUT_LED_COUNT: u32 = 3;
pub const APP_PINOUT_LED_ACTIVE_HIGH: bool =
    APP_PINOUT_OM13087_LED_ACTIVE_HIGH;

pub fn app_pinout_port_decode(code: u32) -> Option<u32> {
    if code < APP_PINOUT_PORT_CODE_BASE {
        return None;
    }
    let port = code - APP_PINOUT_PORT_CODE_BASE;
    if port < HAL_CORE_GPIO_PORT_COUNT {
        Some(port)
    } else {
        None
    }
}

pub fn app_pinout_port_encode(port: u32) -> u32 {
    APP_PINOUT_PORT_CODE_BASE + port
}

pub fn app_pinout_allowed(port: u32, pin: u32) -> bool {
    hal_driver_gpio_pin_valid(port, pin)
        && app_pinout_om13087_allowed(port, pin)
}

pub fn app_pinout_led_get(color: u32) -> Option<(u32, u32)> {
    app_pinout_om13087_led_get(color)
}

pub fn app_pinout_led_level_get(on: bool) -> bool {
    on == APP_PINOUT_LED_ACTIVE_HIGH
}

pub fn app_pinout_led_set(color: u32, on: bool) {
    if let Some((port, pin)) = app_pinout_led_get(color) {
        hal_driver_gpio_write(
            port,
            pin,
            app_pinout_led_level_get(on),
        );
    }
}

fn app_pinout_led_init_one(color: u32) {
    if let Some((port, pin)) = app_pinout_led_get(color) {
        hal_driver_gpio_select(port, pin);
        hal_driver_gpio_output_set(
            port,
            pin,
            app_pinout_led_level_get(false),
        );
    }
}

pub fn app_pinout_init() {
    app_pinout_led_init_one(APP_PINOUT_LED_RED);
    app_pinout_led_init_one(APP_PINOUT_LED_GREEN);
    app_pinout_led_init_one(APP_PINOUT_LED_BLUE);
}
