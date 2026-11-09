// src/hal/driver/clock.rs

use core::sync::atomic::{AtomicU32, Ordering};

use crate::hal::core::*;

const HAL_DRIVER_CLOCK_WAIT_LIMIT: u32 = 100_000;
const HAL_DRIVER_CLOCK_SRC_IRC: u32 = 0;
const HAL_DRIVER_CLOCK_UPDATE_BIT: u32 = 1;
const HAL_DRIVER_CLOCK_AHB_DIV: u32 = 1;
const HAL_DRIVER_CLOCK_FLASH_1CLK_MAX_HZ: u32 = 20_000_000;
const HAL_DRIVER_CLOCK_FLASH_2CLK_MAX_HZ: u32 = 40_000_000;
const HAL_DRIVER_CLOCK_HZ_PER_MHZ: u32 = 1_000_000;

const HAL_DRIVER_CLOCK_BOOT_GATES: u32 = HAL_CORE_CLKCTRL_SYS
    | HAL_CORE_CLKCTRL_ROM
    | HAL_CORE_CLKCTRL_RAM
    | HAL_CORE_CLKCTRL_FLASHREG
    | HAL_CORE_CLKCTRL_FLASHARRAY
    | HAL_CORE_CLKCTRL_GPIO
    | HAL_CORE_CLKCTRL_IOCON;

static HAL_DRIVER_CLOCK_HZ: AtomicU32 =
    AtomicU32::new(HAL_CORE_IRC_HZ);

fn hal_driver_clock_irc_power() {
    hal_core_mmio_clear(
        HAL_CORE_SYSCON_PDRUNCFG,
        HAL_CORE_PDRUN_IRC_OUT | HAL_CORE_PDRUN_IRC,
    );
}

fn hal_driver_clock_flash_cfg_get(hz: u32) -> u32 {
    if hz <= HAL_DRIVER_CLOCK_FLASH_1CLK_MAX_HZ {
        HAL_CORE_FLASHCFG_1CLK
    } else if hz <= HAL_DRIVER_CLOCK_FLASH_2CLK_MAX_HZ {
        HAL_CORE_FLASHCFG_2CLK
    } else {
        HAL_CORE_FLASHCFG_3CLK
    }
}

fn hal_driver_clock_flash_set(hz: u32) {
    hal_core_mmio_modify(
        HAL_CORE_FLASHCFG,
        HAL_CORE_FLASHCFG_MASK,
        hal_driver_clock_flash_cfg_get(hz),
    );
}

fn hal_driver_clock_update_toggle(addr: usize) -> bool {
    hal_core_mmio_write(addr, 0);
    hal_core_mmio_write(addr, HAL_DRIVER_CLOCK_UPDATE_BIT);
    hal_core_mmio_wait_set(
        addr,
        HAL_DRIVER_CLOCK_UPDATE_BIT,
        HAL_DRIVER_CLOCK_WAIT_LIMIT,
    )
}

fn hal_driver_clock_main_select_irc() -> bool {
    hal_core_mmio_write(
        HAL_CORE_SYSCON_MAINCLKSEL,
        HAL_DRIVER_CLOCK_SRC_IRC,
    );
    hal_driver_clock_update_toggle(HAL_CORE_SYSCON_MAINCLKUEN)
}

fn hal_driver_clock_ahb_div_set(div: u32) {
    hal_core_mmio_write(HAL_CORE_SYSCON_SYSAHBCLKDIV, div);
}

pub fn hal_driver_clock_gate_enable(mask: u32) {
    hal_core_mmio_set(HAL_CORE_SYSCON_SYSAHBCLKCTRL, mask);
}

pub fn hal_driver_clock_hz_get() -> u32 {
    HAL_DRIVER_CLOCK_HZ.load(Ordering::Relaxed)
}

pub fn hal_driver_clock_ticks_per_us_get() -> u32 {
    hal_driver_clock_hz_get() / HAL_DRIVER_CLOCK_HZ_PER_MHZ
}

pub fn hal_driver_clock_init() -> bool {
    let hz = HAL_CORE_IRC_HZ / HAL_DRIVER_CLOCK_AHB_DIV;
    hal_driver_clock_irc_power();
    hal_driver_clock_flash_set(hz);
    if !hal_driver_clock_main_select_irc() {
        return false;
    }
    hal_driver_clock_ahb_div_set(HAL_DRIVER_CLOCK_AHB_DIV);
    hal_driver_clock_gate_enable(HAL_DRIVER_CLOCK_BOOT_GATES);
    HAL_DRIVER_CLOCK_HZ.store(hz, Ordering::Relaxed);
    true
}