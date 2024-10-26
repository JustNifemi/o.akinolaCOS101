fn main() {
// listing the variables
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;

// formula for sum and average
	let sum = (2.00*t) + (m) + (3.00*h) + (3.00*d) + (a);
	println!("The sum of the sales is {}", sum);
	let avg = sum/10.00;
	println!("The average of the sales is {}", avg);

}