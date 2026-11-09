// src/hal/boot.rs

use core::ptr::{
    addr_of, addr_of_mut, read_volatile, write_volatile,
};

use core::arch::asm;

use super::core::{
    hal_core_mmio_write, HAL_CORE_IRQ_COUNT, HAL_CORE_NVIC_ICER,
    HAL_CORE_NVIC_ICPR, HAL_CORE_NVIC_ISER,
};

#[allow(non_upper_case_globals)]
extern "C" {
    static _estack: u32;
    static _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
    fn main() -> !;
}

pub type HalBootHook = fn();

#[derive(Clone, Copy)]
pub union HalBootVector {
    handler: unsafe extern "C" fn(),
    stack: *const u32,
    word: u32,
}

unsafe impl Sync for HalBootVector {}

const HAL_BOOT_VECTOR_ZERO: HalBootVector =
    HalBootVector { word: 0 };

const fn hal_boot_vector_handler(
    handler: unsafe extern "C" fn(),
) -> HalBootVector {
    HalBootVector { handler }
}

static mut HAL_BOOT_HOOKS: [Option<HalBootHook>;
    HAL_CORE_IRQ_COUNT as usize] =
    [None; HAL_CORE_IRQ_COUNT as usize];

unsafe fn hal_boot_data_copy() {
    let mut src = addr_of!(_sidata);
    let mut dst = addr_of_mut!(_sdata);
    let end = addr_of_mut!(_edata);
    while dst < end {
        write_volatile(dst, read_volatile(src));
        dst = dst.add(1);
        src = src.add(1);
    }
}

unsafe fn hal_boot_bss_zero() {
    let mut dst = addr_of_mut!(_sbss);
    let end = addr_of_mut!(_ebss);
    while dst < end {
        write_volatile(dst, 0);
        dst = dst.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hal_boot_reset() {
    hal_boot_data_copy();
    hal_boot_bss_zero();
    main()
}

unsafe extern "C" fn hal_boot_fault() {
    loop {}
}

unsafe extern "C" fn hal_boot_nop() {}

fn hal_boot_hook_slot(irq: u32) -> *mut Option<HalBootHook> {
    unsafe {
        (addr_of_mut!(HAL_BOOT_HOOKS) as *mut Option<HalBootHook>)
            .add(irq as usize)
    }
}

fn hal_boot_hook_get(irq: u32) -> Option<HalBootHook> {
    if irq >= HAL_CORE_IRQ_COUNT {
        return None;
    }
    unsafe { read_volatile(hal_boot_hook_slot(irq)) }
}

pub fn hal_boot_hook_set(irq: u32, hook: Option<HalBootHook>) {
    if irq >= HAL_CORE_IRQ_COUNT {
        return;
    }
    unsafe { write_volatile(hal_boot_hook_slot(irq), hook) }
}

pub fn hal_boot_irq_enable(irq: u32) {
    if irq < HAL_CORE_IRQ_COUNT {
        hal_core_mmio_write(HAL_CORE_NVIC_ISER, 1 << irq);
    }
}

pub fn hal_boot_irq_disable(irq: u32) {
    if irq < HAL_CORE_IRQ_COUNT {
        hal_core_mmio_write(HAL_CORE_NVIC_ICER, 1 << irq);
    }
}

fn hal_boot_irq_mask_all() {
    hal_core_mmio_write(HAL_CORE_NVIC_ICER, 0xFFFF_FFFF);
    hal_core_mmio_write(HAL_CORE_NVIC_ICPR, 0xFFFF_FFFF);
}

fn hal_boot_irq_dispatch(irq: u32) {
    match hal_boot_hook_get(irq) {
        Some(hook) => hook(),
        None => hal_boot_irq_disable(irq),
    }
}

unsafe extern "C" fn hal_boot_irq_stub<const IRQ: u32>() {
    hal_boot_irq_dispatch(IRQ);
}

pub fn hal_boot_irq_global_enable() {
    unsafe {
        asm!("cpsie i", options(nostack));
    }
}

pub fn hal_boot_critical_enter() -> u32 {
    let primask: u32;
    unsafe {
        asm!(
            "mrs {0}, PRIMASK",
            "cpsid i",
            out(reg) primask,
            options(nostack)
        );
    }
    primask
}

pub fn hal_boot_critical_exit(primask: u32) {
    if primask & 1 == 0 {
        hal_boot_irq_global_enable();
    }
}

pub fn hal_boot_init() {
    hal_boot_irq_mask_all();
}

#[allow(unused_unsafe)]
#[link_section = ".isr_vector"]
#[no_mangle]
#[used]
pub static HAL_BOOT_VECTORS: [HalBootVector; 48] = [
    HalBootVector { stack: unsafe { addr_of!(_estack) } },
    hal_boot_vector_handler(hal_boot_reset),
    hal_boot_vector_handler(hal_boot_fault),
    hal_boot_vector_handler(hal_boot_fault),
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    hal_boot_vector_handler(hal_boot_nop),
    HAL_BOOT_VECTOR_ZERO,
    HAL_BOOT_VECTOR_ZERO,
    hal_boot_vector_handler(hal_boot_nop),
    hal_boot_vector_handler(hal_boot_nop),
    hal_boot_vector_handler(hal_boot_irq_stub::<0>),
    hal_boot_vector_handler(hal_boot_irq_stub::<1>),
    hal_boot_vector_handler(hal_boot_irq_stub::<2>),
    hal_boot_vector_handler(hal_boot_irq_stub::<3>),
    hal_boot_vector_handler(hal_boot_irq_stub::<4>),
    hal_boot_vector_handler(hal_boot_irq_stub::<5>),
    hal_boot_vector_handler(hal_boot_irq_stub::<6>),
    hal_boot_vector_handler(hal_boot_irq_stub::<7>),
    hal_boot_vector_handler(hal_boot_irq_stub::<8>),
    hal_boot_vector_handler(hal_boot_irq_stub::<9>),
    hal_boot_vector_handler(hal_boot_irq_stub::<10>),
    hal_boot_vector_handler(hal_boot_irq_stub::<11>),
    hal_boot_vector_handler(hal_boot_irq_stub::<12>),
    hal_boot_vector_handler(hal_boot_irq_stub::<13>),
    hal_boot_vector_handler(hal_boot_irq_stub::<14>),
    hal_boot_vector_handler(hal_boot_irq_stub::<15>),
    hal_boot_vector_handler(hal_boot_irq_stub::<16>),
    hal_boot_vector_handler(hal_boot_irq_stub::<17>),
    hal_boot_vector_handler(hal_boot_irq_stub::<18>),
    hal_boot_vector_handler(hal_boot_irq_stub::<19>),
    hal_boot_vector_handler(hal_boot_irq_stub::<20>),
    hal_boot_vector_handler(hal_boot_irq_stub::<21>),
    hal_boot_vector_handler(hal_boot_irq_stub::<22>),
    hal_boot_vector_handler(hal_boot_irq_stub::<23>),
    hal_boot_vector_handler(hal_boot_irq_stub::<24>),
    hal_boot_vector_handler(hal_boot_irq_stub::<25>),
    hal_boot_vector_handler(hal_boot_irq_stub::<26>),
    hal_boot_vector_handler(hal_boot_irq_stub::<27>),
    hal_boot_vector_handler(hal_boot_irq_stub::<28>),
    hal_boot_vector_handler(hal_boot_irq_stub::<29>),
    hal_boot_vector_handler(hal_boot_irq_stub::<30>),
    hal_boot_vector_handler(hal_boot_irq_stub::<31>),
];