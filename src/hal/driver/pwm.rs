// src/hal/driver/pwm.rs

use crate::hal::core::*;
use super::clock::*;
use super::gpio::*;

pub const HAL_DRIVER_PWM_PERIOD: u32 = 1000;

const HAL_DRIVER_PWM_TIMER: u32 = 0;

fn hal_driver_pwm_write(reg: usize, value: u32) {
    hal_core_mmio_write(
        hal_core_timer_base_get(HAL_DRIVER_PWM_TIMER) + reg,
        value,
    );
}

fn hal_driver_pwm_match_get(high_ticks: u32) -> u32 {
    if high_ticks >= HAL_DRIVER_PWM_PERIOD {
        HAL_DRIVER_PWM_PERIOD + 1
    } else if high_ticks == 0 {
        1
    } else {
        high_ticks
    }
}

pub fn hal_driver_pwm_high_set(high_ticks: u32) {
    hal_driver_pwm_write(
        HAL_CORE_TIMER_MR0,
        hal_driver_pwm_match_get(high_ticks),
    );
}

pub fn hal_driver_pwm_init(port: u32, pin: u32, func: u32) -> bool {
    let ticks = hal_driver_clock_ticks_per_us_get();
    if ticks == 0 {
        return false;
    }
    hal_driver_clock_gate_enable(HAL_CORE_CLKCTRL_CT16B0);
    hal_driver_pwm_write(HAL_CORE_TIMER_TCR, 0);
    hal_driver_pwm_write(HAL_CORE_TIMER_CTCR, 0);
    hal_driver_pwm_write(HAL_CORE_TIMER_PR, ticks - 1);
    hal_driver_pwm_write(
        HAL_CORE_TIMER_MR3,
        HAL_DRIVER_PWM_PERIOD - 1,
    );
    hal_driver_pwm_high_set(HAL_DRIVER_PWM_PERIOD);
    hal_driver_pwm_write(
        HAL_CORE_TIMER_MCR,
        HAL_CORE_TIMER_MCR_MR3R,
    );
    hal_driver_pwm_write(
        HAL_CORE_TIMER_EMR,
        HAL_CORE_TIMER_EMR_EM0,
    );
    hal_driver_pwm_write(
        HAL_CORE_TIMER_PWMC,
        HAL_CORE_TIMER_PWMC_EN0,
    );
    hal_driver_pwm_write(
        HAL_CORE_TIMER_TCR,
        HAL_CORE_TIMER_TCR_RESET,
    );
    hal_driver_pwm_write(
        HAL_CORE_TIMER_TCR,
        HAL_CORE_TIMER_TCR_EN,
    );
    hal_driver_gpio_function_set(port, pin, func)
}