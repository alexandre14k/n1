// src/app/panic.rs

use core::panic::PanicInfo;

fn app_panic_irq_off() {
    unsafe {
        core::arch::asm!("cpsid i", options(nomem, nostack));
    }
}

fn app_panic_halt() -> ! {
    loop {}
}

#[panic_handler]
fn app_panic_handler(_info: &PanicInfo) -> ! {
    app_panic_irq_off();
    app_panic_halt()
}