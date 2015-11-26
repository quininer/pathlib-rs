pathlib-rs
----------

example:

	extern crate pathlib;

	use std::fs::File;
	use pathlib::PathDiv;

	fn main() {
	    let root = PathDiv::new("/");
	    let tmp = root / "tmp" / "test.txt";
	    File::create(&tmp).expect("fail.");

	    println!(
	        "{} exists: {}",
	        tmp.path().display(),
	        tmp.path().exists()
	    );
	}
