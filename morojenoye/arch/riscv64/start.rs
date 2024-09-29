use core::arch::{asm, global_asm};

use crate::interrupt;

extern "Rust" {
	fn main(a0: usize, a1: usize) -> !;
}

global_asm!(
	".section .text.start",
	".globl start",
	"start:",
	"la sp, _stack_0",
	"j setup",
);

// Table 18. HSM Hart Start Register State
// +---------------+-----------------------------------+
// | Register Name | Register Value                    |
// +---------------+-----------------------------------+
// | satp          | 0                                 |
// | sstatus.SIE   | 0                                 |
// | a0            | hartid                            |
// | a1            | opaque                            |
// +---------------+-----------------------------------+
// | All other registers remain in an undefined state. |
// +---------------+-----------------------------------+
#[no_mangle]
unsafe fn setup(a0: usize, a1: usize) {
	asm!(
		".option push",
		".option norelax",
		"la gp, __global_pointer$",
		".option pop"
	);
	interrupt::setup();
	main(a0, a1);
}
