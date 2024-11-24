// Program to show staff incentives

use std::io;

// listing variables

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("PAU Staff Publication Incentive System");
	println!("Entry number:");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let sn:u32 = input3.trim().parse().expect("Not a valid number");

	println!("Enter your name");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Enter the number of papers you have published");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let num:u32 = input2.trim().parse().expect("Not a valid number");

	//determining the incentives using if else (decision making)
	// entry no 101 will not run

	println!("\n");

	if num >=3 && num <=5 && sn <=100{
		println!("Staff Name: {}", input1);
		println!("Incentive = N500,000");
	} else if num >5 && num < 10 && sn <=100{
		println!("Staff Name: {}", input1);
		println!("Incentive = N800,000");
	} else if num >10 && sn <=100 {
		println!("Staff Name: {}", input1);
		println!("Incentive = N1,000,000");
	} else if num < 3 && sn <=100 {
		println!("Staff Name: {}", input1);
		println!("Incentive = N100,000");
	}
}