// src/hal/driver/timer.rs

use crate::hal::core::*;
use super::clock::*;

pub const HAL_DRIVER_TIMER_CT32B0: u32 = 2;
pub const HAL_DRIVER_TIMER_CT32B1: u32 = 3;

const HAL_DRIVER_TIMER_CTCR_TIMER: u32 = 0;
const HAL_DRIVER_TIMER_MCR_ONESHOT: u32 =
    HAL_CORE_TIMER_MCR_MR0I | HAL_CORE_TIMER_MCR_MR0S;

fn hal_driver_timer_valid(index: u32) -> bool {
    index == HAL_DRIVER_TIMER_CT32B0
        || index == HAL_DRIVER_TIMER_CT32B1
}

fn hal_driver_timer_reg_get(index: u32, reg: usize) -> usize {
    hal_core_timer_base_get(index) + reg
}

fn hal_driver_timer_write(index: u32, reg: usize, value: u32) {
    hal_core_mmio_write(
        hal_driver_timer_reg_get(index, reg),
        value,
    );
}

fn hal_driver_timer_read(index: u32, reg: usize) -> u32 {
    hal_core_mmio_read(hal_driver_timer_reg_get(index, reg))
}

fn hal_driver_timer_prescale_get() -> Option<u32> {
    let ticks = hal_driver_clock_ticks_per_us_get();
    if ticks == 0 {
        None
    } else {
        Some(ticks - 1)
    }
}

pub fn hal_driver_timer_gate_get(index: u32) -> u32 {
    if index == HAL_DRIVER_TIMER_CT32B0 {
        HAL_CORE_CLKCTRL_CT32B0
    } else {
        HAL_CORE_CLKCTRL_CT32B1
    }
}

pub fn hal_driver_timer_irq_get(index: u32) -> u32 {
    if index == HAL_DRIVER_TIMER_CT32B0 {
        HAL_CORE_IRQ_CT32B0
    } else {
        HAL_CORE_IRQ_CT32B1
    }
}

pub fn hal_driver_timer_stop(index: u32) {
    if hal_driver_timer_valid(index) {
        hal_driver_timer_write(index, HAL_CORE_TIMER_TCR, 0);
    }
}

pub fn hal_driver_timer_irq_clear(index: u32) {
    if hal_driver_timer_valid(index) {
        hal_driver_timer_write(
            index,
            HAL_CORE_TIMER_IR,
            HAL_CORE_TIMER_IR_MR0,
        );
    }
}

pub fn hal_driver_timer_running(index: u32) -> bool {
    hal_driver_timer_valid(index)
        && hal_driver_timer_read(index, HAL_CORE_TIMER_TCR)
            & HAL_CORE_TIMER_TCR_EN
            != 0
}

pub fn hal_driver_timer_init(index: u32) -> bool {
    if !hal_driver_timer_valid(index) {
        return false;
    }
    hal_driver_clock_gate_enable(hal_driver_timer_gate_get(index));
    hal_driver_timer_stop(index);
    hal_driver_timer_write(
        index,
        HAL_CORE_TIMER_CTCR,
        HAL_DRIVER_TIMER_CTCR_TIMER,
    );
    hal_driver_timer_irq_clear(index);
    true
}

pub fn hal_driver_timer_oneshot_start(
    index: u32,
    us: u32,
) -> bool {
    if !hal_driver_timer_valid(index) || us == 0 {
        return false;
    }
    let prescale = match hal_driver_timer_prescale_get() {
        Some(value) => value,
        None => return false,
    };
    hal_driver_timer_stop(index);
    hal_driver_timer_write(
        index,
        HAL_CORE_TIMER_TCR,
        HAL_CORE_TIMER_TCR_RESET,
    );
    hal_driver_timer_write(index, HAL_CORE_TIMER_PR, prescale);
    hal_driver_timer_write(index, HAL_CORE_TIMER_MR0, us);
    hal_driver_timer_write(
        index,
        HAL_CORE_TIMER_MCR,
        HAL_DRIVER_TIMER_MCR_ONESHOT,
    );
    hal_driver_timer_irq_clear(index);
    hal_driver_timer_write(
        index,
        HAL_CORE_TIMER_TCR,
        HAL_CORE_TIMER_TCR_EN,
    );
    true
}