#[no_mangle]
#[link_section = ".text.start_panic"]
fn start_panic() {
	loop {}
}
