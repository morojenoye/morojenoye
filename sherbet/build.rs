use std::{env, fs, path::PathBuf};

fn main() {
	let outputs = PathBuf::from(env::var("OUT_DIR").unwrap());

	fs::write(outputs.join("layout.ld"), include_bytes!("layout.ld")).unwrap();
	println!("cargo:rerun-if-changed=layout.ld");

	println!("cargo:rustc-link-search={}", outputs.display());
	println!("cargo:rerun-if-changed=build.rs");
}
