#![no_main]
#![no_std]
extern crate alloc;
extern crate morojenoye;

use alloc::string::String;
use core::{fmt::Write, panic::PanicInfo};

use morojenoye::{allocator, drivers::ser};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
unsafe fn main(_a0: usize, a1: usize) -> ! {
	let fdt = fdt::Fdt::from_ptr(a1 as *const u8).unwrap();
	ser::setup(&fdt).unwrap();
	allocator::setup();

	let dummy = String::from("123");
	write!(ser::IO.get_mut().unwrap(), "{}", dummy).unwrap();
	loop {}
}
