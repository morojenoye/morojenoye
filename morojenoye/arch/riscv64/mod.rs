use core::arch::asm;

pub unsafe fn setup_panic() {
	asm!("la t0, start_panic");
	asm!("csrw stvec, t0");
}

#[no_mangle]
#[link_section = ".text.start_panic"]
fn start_panic() {
	loop {}
}
