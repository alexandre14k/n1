// src/app/statemachine.rs

use core::sync::atomic::{AtomicU32, Ordering};

use crate::hal::*;
use super::feature::*;
use super::interrupt::*;
use super::pinout::*;
use super::queue::*;

const APP_STATEMACHINE_STATE_FAULT: u32 = 0;
const APP_STATEMACHINE_STATE_BOOT_BLUE: u32 = 1;
const APP_STATEMACHINE_STATE_WAIT_GREEN: u32 = 2;
const APP_STATEMACHINE_STATE_GREEN: u32 = 3;
const APP_STATEMACHINE_STATE_WAIT_RED: u32 = 4;
const APP_STATEMACHINE_STATE_RED_FIRST: u32 = 5;
const APP_STATEMACHINE_STATE_RED_GAP: u32 = 6;
const APP_STATEMACHINE_STATE_RED_SECOND: u32 = 7;
const APP_STATEMACHINE_STATE_WAIT_LOOP: u32 = 8;

const APP_STATEMACHINE_BLINK_US: u32 = 100_000;
const APP_STATEMACHINE_GAP_US: u32 = 100_000;
const APP_STATEMACHINE_FIRST_WAIT_US: u32 = 2_000_000;
const APP_STATEMACHINE_WAIT_US: u32 = 1_000_000;

static APP_STATEMACHINE_STATE: AtomicU32 =
    AtomicU32::new(APP_STATEMACHINE_STATE_FAULT);

fn app_statemachine_state_get() -> u32 {
    APP_STATEMACHINE_STATE.load(Ordering::Relaxed)
}

fn app_statemachine_state_set(state: u32) {
    APP_STATEMACHINE_STATE.store(state, Ordering::Relaxed);
}

fn app_statemachine_fault() {
    app_statemachine_state_set(APP_STATEMACHINE_STATE_FAULT);
    app_pinout_led_set(APP_PINOUT_LED_RED, true);
}

fn app_statemachine_led_pulse(color: u32, us: u32) -> bool {
    match app_pinout_led_get(color) {
        Some((port, pin)) => app_feature_pulse_start(
            port,
            pin,
            APP_PINOUT_LED_ACTIVE_HIGH,
            us,
        ),
        None => false,
    }
}

fn app_statemachine_wait_start(us: u32) -> bool {
    hal_driver_timer_oneshot_start(
        APP_INTERRUPT_TIMER_WAIT,
        us,
    )
}

fn app_statemachine_wait_enter(state: u32, us: u32) {
    app_statemachine_state_set(state);

    if !app_statemachine_wait_start(us) {
        app_statemachine_fault();
    }
}

fn app_statemachine_blink_enter(state: u32, color: u32) {
    app_statemachine_state_set(state);

    if !app_statemachine_led_pulse(
        color,
        APP_STATEMACHINE_BLINK_US,
    ) {
        app_statemachine_fault();
    }
}

fn app_statemachine_pulse_done() {
    app_feature_pulse_end();

    match app_statemachine_state_get() {
        APP_STATEMACHINE_STATE_BOOT_BLUE => {
            app_statemachine_wait_enter(
                APP_STATEMACHINE_STATE_WAIT_GREEN,
                APP_STATEMACHINE_FIRST_WAIT_US,
            );
        }

        APP_STATEMACHINE_STATE_GREEN => {
            app_statemachine_wait_enter(
                APP_STATEMACHINE_STATE_WAIT_RED,
                APP_STATEMACHINE_WAIT_US,
            );
        }

        APP_STATEMACHINE_STATE_RED_FIRST => {
            app_statemachine_wait_enter(
                APP_STATEMACHINE_STATE_RED_GAP,
                APP_STATEMACHINE_GAP_US,
            );
        }

        APP_STATEMACHINE_STATE_RED_SECOND => {
            app_statemachine_wait_enter(
                APP_STATEMACHINE_STATE_WAIT_LOOP,
                APP_STATEMACHINE_WAIT_US,
            );
        }

        _ => {}
    }
}

fn app_statemachine_wait_done() {
    match app_statemachine_state_get() {
        APP_STATEMACHINE_STATE_WAIT_GREEN
        | APP_STATEMACHINE_STATE_WAIT_LOOP => {
            app_statemachine_blink_enter(
                APP_STATEMACHINE_STATE_GREEN,
                APP_PINOUT_LED_GREEN,
            );
        }

        APP_STATEMACHINE_STATE_WAIT_RED => {
            app_statemachine_blink_enter(
                APP_STATEMACHINE_STATE_RED_FIRST,
                APP_PINOUT_LED_RED,
            );
        }

        APP_STATEMACHINE_STATE_RED_GAP => {
            app_statemachine_blink_enter(
                APP_STATEMACHINE_STATE_RED_SECOND,
                APP_PINOUT_LED_RED,
            );
        }

        _ => {}
    }
}

fn app_statemachine_event(item: AppQueueItem) {
    match item.origin {
        APP_QUEUE_ORIGIN_PULSE => {
            app_statemachine_pulse_done();
        }

        APP_QUEUE_ORIGIN_TIMER => {
            app_statemachine_wait_done();
        }

        _ => {}
    }
}

fn app_statemachine_peripheral_init() -> bool {
    if !hal_driver_clock_init() {
        return false;
    }

    app_pinout_init();

    hal_driver_timer_init(APP_INTERRUPT_TIMER_PULSE)
        && hal_driver_timer_init(APP_INTERRUPT_TIMER_WAIT)
}

pub fn app_statemachine_init() {
    if !app_statemachine_peripheral_init() {
        app_statemachine_fault();
        return;
    }

    app_queue_reset();
    app_interrupt_init();

    hal_boot_irq_global_enable();

    app_statemachine_blink_enter(
        APP_STATEMACHINE_STATE_BOOT_BLUE,
        APP_PINOUT_LED_BLUE,
    );
}

pub fn app_statemachine_step() {
    if let Some(item) = app_queue_pop() {
        app_statemachine_event(item);
    }
}
