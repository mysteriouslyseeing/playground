#![no_std]

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
