#![no_main]
#![no_std]
extern crate morojenoye;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
fn main(a0: usize, a1: usize) -> ! {
	loop {}
}
