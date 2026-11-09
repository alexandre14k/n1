// src/app/feature/pulse.rs

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::hal::*;
use super::super::interrupt::*;
use super::super::pinout::*;

static APP_FEATURE_PULSE_BUSY: AtomicBool = AtomicBool::new(false);
static APP_FEATURE_PULSE_PORT: AtomicU32 = AtomicU32::new(0);
static APP_FEATURE_PULSE_PIN: AtomicU32 = AtomicU32::new(0);
static APP_FEATURE_PULSE_IDLE: AtomicBool = AtomicBool::new(false);

fn app_feature_pulse_claim(port: u32, pin: u32, idle: bool) {
    APP_FEATURE_PULSE_PORT.store(port, Ordering::Relaxed);
    APP_FEATURE_PULSE_PIN.store(pin, Ordering::Relaxed);
    APP_FEATURE_PULSE_IDLE.store(idle, Ordering::Relaxed);
    APP_FEATURE_PULSE_BUSY.store(true, Ordering::Release);
}

fn app_feature_pulse_drive(port: u32, pin: u32, level: bool) {
    hal_driver_gpio_select(port, pin);
    hal_driver_gpio_output_set(port, pin, level);
}

fn app_feature_pulse_idle_write() {
    hal_driver_gpio_write(
        APP_FEATURE_PULSE_PORT.load(Ordering::Relaxed),
        APP_FEATURE_PULSE_PIN.load(Ordering::Relaxed),
        APP_FEATURE_PULSE_IDLE.load(Ordering::Relaxed),
    );
}

fn app_feature_pulse_release() {
    app_feature_pulse_idle_write();
    APP_FEATURE_PULSE_BUSY.store(false, Ordering::Release);
}

pub fn app_feature_pulse_busy() -> bool {
    APP_FEATURE_PULSE_BUSY.load(Ordering::Acquire)
}

pub fn app_feature_pulse_start(
    port: u32,
    pin: u32,
    active_high: bool,
    width_us: u32,
) -> bool {
    if app_feature_pulse_busy()
        || width_us == 0
        || !app_pinout_allowed(port, pin)
    {
        return false;
    }
    app_feature_pulse_claim(port, pin, !active_high);
    app_feature_pulse_drive(port, pin, active_high);
    if hal_driver_timer_oneshot_start(
        APP_INTERRUPT_TIMER_PULSE,
        width_us,
    ) {
        return true;
    }
    app_feature_pulse_release();
    false
}

pub fn app_feature_pulse_end() {
    if !app_feature_pulse_busy() {
        return;
    }
    hal_driver_timer_stop(APP_INTERRUPT_TIMER_PULSE);
    app_feature_pulse_release();
}