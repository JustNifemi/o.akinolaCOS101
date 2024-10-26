fn main(){
	// variables
	let p:f64=210_000.00;
	let r:f64=5.00;
	let n:f64=3.00;

	//formula for amount
	let a=p*(1.00-(r/100.00)) .powf(n);
	println!("The value of the TV after 3 years is {}", a);
	let d=p-a;
	println!("The depreciation is {}", d );

}