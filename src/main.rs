use std::time::Instant;

use a001415::foldings;

mod a001415;

fn main() {
	// let mut prev = Instant::now();
	// for (i, n) in A001415::default().enumerate() {
	// 	println!("{i:>2}: {n} (took {:?})", prev.elapsed());
	// 	prev = Instant::now();
	// }

	// let n = 5;

	let time_start = Instant::now();
	let value = foldings(&[12, 2]);
	println!("{value} (took {:?})", time_start.elapsed());
}
