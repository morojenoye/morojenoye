use core::arch::asm;

pub type Flags = u8;

pub unsafe fn setup(flags: Flags) {
	asm!("csrrs zero, sstatus, {rs1}",
		rs1 = in(reg) flags,
		options(preserves_flags, nostack),
	);
}

pub unsafe fn block() -> Flags {
	let flags: Flags;
	asm!("csrrci {rd}, sstatus, 0b10",
		rd = out(reg) flags,
		options(preserves_flags, nostack)
	);
	flags
}
