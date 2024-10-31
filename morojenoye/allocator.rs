use core::mem::MaybeUninit;

use embedded_alloc::LlffHeap as Heap;
use riscv as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// =========================================================================

const HEAP_SPACE_SIZE: usize = 1024 * 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SPACE_SIZE] =
	[MaybeUninit::uninit(); HEAP_SPACE_SIZE];

pub fn setup() {
	unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SPACE_SIZE) }
}
