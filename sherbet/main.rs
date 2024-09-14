#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
#[link_section = ".text.start"]
unsafe fn start() {
	asm!(
		".option push",
		".option norelax",
		"la gp, __global_pointer$",
		".option pop"
	);
	asm!("la sp, _stack_0");
}
