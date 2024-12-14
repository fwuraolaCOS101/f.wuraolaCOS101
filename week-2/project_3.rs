fn main() {
	let p:u64 = 510_000;
	let r:u64 = 5;
	let n:u64 = 3;

	let a = p * (1 - (r / 100)) * n;
	let ci = a - p;

	println!("The compound interest is {}",ci);

}