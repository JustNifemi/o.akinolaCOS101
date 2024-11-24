//first question
// Program to show elidgibility for voting for the student council
use std::io;

//For data inputation
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("PAU Student Council Voter System");
	println!("Please fill the form as directed");
	println!("Entry number:");
	io::stdin().read_line(&mut input8).expect("Not a valid string");
	let sn:u32 = input8.trim().parse().expect("Not a valid number");

	println!("Name:");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Email:");
	io::stdin().read_line(&mut input2).expect("Not a valid string");

	println!("Department:");
	io::stdin().read_line(&mut input3).expect("Not a valid string");

	println!("State of Origin:");
	io::stdin().read_line(&mut input4).expect("Not a valid string");

	println!("I am a Class Rep. (type in true or false)");
	io::stdin().read_line(&mut input5).expect("Not a valid string");
	let _stat:bool = input5.trim().parse().expect("Invalid response");

	println!("Level (100/200/300/400/500):");
	io::stdin().read_line(&mut input6).expect("Not a valid string");
	let _lvl:u32 = input6.trim().parse().expect("Not a valid number");

	println!("CGPA:");
	io::stdin().read_line(&mut input7).expect("Not a valid string");
	let _cgpa:f32 = input7.trim().parse().expect("Not a valid number");

	// for eligibility(decision making)
	// entry no 51 will not run 

	if _stat == true && _lvl > 199 && _cgpa > 4.0 && sn <=50{
		println!("Candidate Details:");
		println!("Name: {}", input1);
		println!("Email: {}", input2);
		println!("Department: {}", input3);
		println!("State of Origin: {}", input4);

		println!("You can vote");
	}
	else if _stat == true && _lvl == 100 && sn<= 50 {
		println!("Sorry, you are not eligible to vote");
	}
	else {
		println!("Sorry, you are not eligible to vote");
	}
}


// second question. do not run together

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