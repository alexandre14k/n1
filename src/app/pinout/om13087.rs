// src/app/pinout/om13087.rs

pub const APP_PINOUT_OM13087_LED_ACTIVE_HIGH: bool = false;

const APP_PINOUT_OM13087_LED: [(u32, u32); 3] = [
    (0, 7),
    (0, 8),
    (0, 9),
];

const APP_PINOUT_OM13087_RESERVED: [(u32, u32); 5] = [
    (0, 0),
    (0, 10),
    (1, 3),
    (0, 4),
    (0, 5),
];

fn app_pinout_om13087_pair_find(
    list: &[(u32, u32)],
    port: u32,
    pin: u32,
) -> Option<usize> {
    let mut index = 0;
    while index < list.len() {
        if list[index].0 == port && list[index].1 == pin {
            return Some(index);
        }
        index += 1;
    }
    None
}

pub fn app_pinout_om13087_led_get(color: u32) -> Option<(u32, u32)> {
    APP_PINOUT_OM13087_LED.get(color as usize).copied()
}

pub fn app_pinout_om13087_reserved(port: u32, pin: u32) -> bool {
    app_pinout_om13087_pair_find(
        &APP_PINOUT_OM13087_RESERVED,
        port,
        pin,
    )
    .is_some()
}

pub fn app_pinout_om13087_allowed(port: u32, pin: u32) -> bool {
    !app_pinout_om13087_reserved(port, pin)
}
