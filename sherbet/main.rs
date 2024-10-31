#![no_main]
#![no_std]
extern crate morojenoye;

use core::{fmt::Write, panic::PanicInfo};

use morojenoye::drivers::ser;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
unsafe fn main(_a0: usize, a1: usize) -> ! {
	let fdt = fdt::Fdt::from_ptr(a1 as *const u8).unwrap();
	ser::setup(&fdt).unwrap();

	write!(ser::IO.get_mut().unwrap(), "123").unwrap();
	loop {}
}
