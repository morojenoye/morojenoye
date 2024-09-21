use {crate::arch::interrupt, core::marker::PhantomData};

pub struct Guard {
	inner: PhantomData<*mut ()>,
	flags: interrupt::Flags,
}

impl Drop for Guard {
	fn drop(&mut self) {
		unsafe { interrupt::setup(self.flags) };
	}
}

pub fn block() -> Guard {
	let flags = unsafe { interrupt::block() };
	let inner = PhantomData;
	Guard { inner, flags }
}
