// src/app/interrupt/timer.rs

use crate::hal::*;
use super::super::queue::*;
use super::app_interrupt_hook_register;

pub const APP_INTERRUPT_TIMER_PULSE: u32 = HAL_DRIVER_TIMER_CT32B0;
pub const APP_INTERRUPT_TIMER_WAIT: u32 = HAL_DRIVER_TIMER_CT32B1;

fn app_interrupt_timer_pulse_isr() {
    hal_driver_timer_irq_clear(APP_INTERRUPT_TIMER_PULSE);
    app_queue_push_isr(APP_QUEUE_ORIGIN_PULSE, 0);
}

fn app_interrupt_timer_wait_isr() {
    hal_driver_timer_irq_clear(APP_INTERRUPT_TIMER_WAIT);
    app_queue_push_isr(APP_QUEUE_ORIGIN_TIMER, 0);
}

pub fn app_interrupt_timer_init() {
    app_interrupt_hook_register(
        hal_driver_timer_irq_get(APP_INTERRUPT_TIMER_PULSE),
        app_interrupt_timer_pulse_isr,
    );
    app_interrupt_hook_register(
        hal_driver_timer_irq_get(APP_INTERRUPT_TIMER_WAIT),
        app_interrupt_timer_wait_isr,
    );
}