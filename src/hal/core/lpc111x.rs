// src/hal/core/lpc111x.rs

#![cfg(feature = "lpc111x")]

pub const HAL_CORE_SYSCON_BASE: usize = 0x4004_8000;
pub const HAL_CORE_IOCON_BASE: usize = 0x4004_4000;
pub const HAL_CORE_FLASHCTRL_BASE: usize = 0x4003_C000;
pub const HAL_CORE_CT32B0_BASE: usize = 0x4001_4000;
pub const HAL_CORE_CT32B1_BASE: usize = 0x4001_8000;
pub const HAL_CORE_GPIO0_BASE: usize = 0x5000_0000;
pub const HAL_CORE_GPIO1_BASE: usize = 0x5001_0000;
pub const HAL_CORE_GPIO2_BASE: usize = 0x5002_0000;
pub const HAL_CORE_GPIO3_BASE: usize = 0x5003_0000;

pub const HAL_CORE_SYSCON_PDRUNCFG: usize =
    HAL_CORE_SYSCON_BASE + 0x238;
pub const HAL_CORE_SYSCON_MAINCLKSEL: usize =
    HAL_CORE_SYSCON_BASE + 0x070;
pub const HAL_CORE_SYSCON_MAINCLKUEN: usize =
    HAL_CORE_SYSCON_BASE + 0x074;
pub const HAL_CORE_SYSCON_SYSAHBCLKDIV: usize =
    HAL_CORE_SYSCON_BASE + 0x078;
pub const HAL_CORE_SYSCON_SYSAHBCLKCTRL: usize =
    HAL_CORE_SYSCON_BASE + 0x080;

pub const HAL_CORE_CLKCTRL_SYS: u32 = 1 << 0;
pub const HAL_CORE_CLKCTRL_ROM: u32 = 1 << 1;
pub const HAL_CORE_CLKCTRL_RAM: u32 = 1 << 2;
pub const HAL_CORE_CLKCTRL_FLASHREG: u32 = 1 << 3;
pub const HAL_CORE_CLKCTRL_FLASHARRAY: u32 = 1 << 4;
pub const HAL_CORE_CLKCTRL_GPIO: u32 = 1 << 6;
pub const HAL_CORE_CLKCTRL_CT32B0: u32 = 1 << 9;
pub const HAL_CORE_CLKCTRL_CT32B1: u32 = 1 << 10;
pub const HAL_CORE_CLKCTRL_IOCON: u32 = 1 << 16;

pub const HAL_CORE_PDRUN_IRC_OUT: u32 = 1 << 0;
pub const HAL_CORE_PDRUN_IRC: u32 = 1 << 1;

pub const HAL_CORE_FLASHCFG: usize =
    HAL_CORE_FLASHCTRL_BASE + 0x010;
pub const HAL_CORE_FLASHCFG_MASK: u32 = 0x3;
pub const HAL_CORE_FLASHCFG_1CLK: u32 = 0x0;
pub const HAL_CORE_FLASHCFG_2CLK: u32 = 0x1;
pub const HAL_CORE_FLASHCFG_3CLK: u32 = 0x2;

pub const HAL_CORE_GPIO_DIR: usize = 0x8000;

pub const HAL_CORE_TIMER_IR: usize = 0x00;
pub const HAL_CORE_TIMER_TCR: usize = 0x04;
pub const HAL_CORE_TIMER_PR: usize = 0x0C;
pub const HAL_CORE_TIMER_MCR: usize = 0x14;
pub const HAL_CORE_TIMER_MR0: usize = 0x18;
pub const HAL_CORE_TIMER_CTCR: usize = 0x70;

pub const HAL_CORE_TIMER_TCR_EN: u32 = 1 << 0;
pub const HAL_CORE_TIMER_TCR_RESET: u32 = 1 << 1;
pub const HAL_CORE_TIMER_MCR_MR0I: u32 = 1 << 0;
pub const HAL_CORE_TIMER_MCR_MR0S: u32 = 1 << 2;
pub const HAL_CORE_TIMER_IR_MR0: u32 = 1 << 0;

pub const HAL_CORE_NVIC_ISER: usize = 0xE000_E100;
pub const HAL_CORE_NVIC_ICER: usize = 0xE000_E180;
pub const HAL_CORE_NVIC_ICPR: usize = 0xE000_E280;

pub const HAL_CORE_IRQ_CT32B0: u32 = 18;
pub const HAL_CORE_IRQ_CT32B1: u32 = 19;
pub const HAL_CORE_IRQ_COUNT: u32 = 32;

pub const HAL_CORE_IRC_HZ: u32 = 12_000_000;

pub fn hal_core_gpio_base_get(port: u32) -> usize {
    match port {
        0 => HAL_CORE_GPIO0_BASE,
        1 => HAL_CORE_GPIO1_BASE,
        2 => HAL_CORE_GPIO2_BASE,
        _ => HAL_CORE_GPIO3_BASE,
    }
}

pub fn hal_core_timer_base_get(index: u32) -> usize {
    match index {
        2 => HAL_CORE_CT32B0_BASE,
        _ => HAL_CORE_CT32B1_BASE,
    }
}
