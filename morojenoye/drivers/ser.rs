use core::{cell::OnceCell, fmt};

use fdt::{node::FdtNode, standard_nodes::MemoryRegion, Fdt};

// =========================================================================

pub static mut IO: OnceCell<UART> = OnceCell::new();

pub enum UART {
	NS16550A(uart_16550::MmioSerialPort),
	SiFive(mmio_sifive_uart::UART),
}

impl fmt::Write for UART {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		match self {
			UART::NS16550A(u) => u.write_str(s),
			UART::SiFive(u) => u.write_str(s),
		}
	}
}

// =========================================================================

fn try_register_dnode(dcompat: &str, reg: MemoryRegion) -> Option<()> {
	let (addr, size) = (reg.starting_address as usize, reg.size.unwrap());

	let ser = if dcompat == "ns16550a" {
		UART::NS16550A(unsafe { uart_16550::MmioSerialPort::new(addr) })
	} else if dcompat == "sifive,uart0" {
		UART::SiFive(mmio_sifive_uart::UART::new(addr, size).unwrap())
	} else {
		return None;
	};
	unsafe { IO.set(ser).ok() }
}

fn try_find_driver(dnode: FdtNode, reg: MemoryRegion) -> Option<()> {
	let mut all = dnode.compatible()?.all();
	all.find_map(|dcompat| try_register_dnode(dcompat, reg))
}

pub fn setup(fdt: &Fdt) -> Option<()> {
	// Now we take only first node to treat that as stdio.
	let dnode = fdt.find_all_nodes("/soc/serial").next()?;
	try_find_driver(dnode, dnode.reg()?.next()?)
}
