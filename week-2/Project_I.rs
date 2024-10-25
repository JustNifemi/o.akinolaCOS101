fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

// compound intrest
	let a = p * ( 1.0 + ( r/100.0 )) .powf(n);
	println!("Amount is {}", a );
	let ci = a - p;
	println!("Compound Intrest is {}", ci );

}