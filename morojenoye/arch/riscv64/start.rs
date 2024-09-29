use {crate::interrupt, core::arch::asm};

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
#[link_section = ".text.start"]
unsafe fn start() {
	asm!(
		".option push",
		".option norelax",
		"la gp, __global_pointer$",
		".option pop"
	);
	asm!("la sp, _stack_0");

	interrupt::setup();
	extern "Rust" {
		fn main() -> !;
	}
	main();
}
