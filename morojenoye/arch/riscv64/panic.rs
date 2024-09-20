use core::arch::asm;

pub unsafe fn setup() {
	asm!("la t0, start_panic");
	asm!("csrw stvec, t0");
}
