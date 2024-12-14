fn main() {
	
	let p:u64 = 520_000_000;
	let n:u64 = 5;
	let r:u64 = 10;

	let a = p * (1 + (r/100)) * n;
	let cl = a - p;

	println!("The compound interest is {}",cl);
}