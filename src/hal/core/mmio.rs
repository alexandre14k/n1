// src/hal/core/mmio.rs

use core::ptr::{read_volatile, write_volatile};

#[inline(always)]
pub fn hal_core_mmio_read(addr: usize) -> u32 {
    unsafe { read_volatile(addr as *const u32) }
}

#[inline(always)]
pub fn hal_core_mmio_write(addr: usize, value: u32) {
    unsafe { write_volatile(addr as *mut u32, value) }
}

#[inline(always)]
pub fn hal_core_mmio_set(addr: usize, mask: u32) {
    let value = hal_core_mmio_read(addr);
    hal_core_mmio_write(addr, value | mask);
}

#[inline(always)]
pub fn hal_core_mmio_clear(addr: usize, mask: u32) {
    let value = hal_core_mmio_read(addr);
    hal_core_mmio_write(addr, value & !mask);
}

#[inline(always)]
pub fn hal_core_mmio_modify(addr: usize, mask: u32, bits: u32) {
    let value = hal_core_mmio_read(addr);
    hal_core_mmio_write(addr, (value & !mask) | (bits & mask));
}

pub fn hal_core_mmio_wait_set(
    addr: usize,
    mask: u32,
    limit: u32,
) -> bool {
    let mut count = limit;
    while count != 0 {
        if hal_core_mmio_read(addr) & mask == mask {
            return true;
        }
        count -= 1;
    }
    false
}

pub fn hal_core_mmio_wait_clear(
    addr: usize,
    mask: u32,
    limit: u32,
) -> bool {
    let mut count = limit;
    while count != 0 {
        if hal_core_mmio_read(addr) & mask == 0 {
            return true;
        }
        count -= 1;
    }
    false
}