// src/app/interrupt.rs

mod timer;

pub use self::timer::*;

use crate::hal::*;

pub fn app_interrupt_hook_register(irq: u32, hook: HalBootHook) {
    hal_boot_hook_set(irq, Some(hook));
    hal_boot_irq_enable(irq);
}

pub fn app_interrupt_init() {
    app_interrupt_timer_init();
}