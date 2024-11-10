// Rust program to determine annual incentives with conditions
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter your age");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    println!("Program to Determine Annual Incentives");
    println!("You are an experienced worker. true/false");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let exp:bool = input1.trim().parse().expect("Invalid response");


    if exp == true && age>= 40 {
        println!("Your incentive is N1,560,000");
    } else if exp == true && age>= 30 && age <40 {
        println!("Your incentive is N1,480,000");
    } else if exp == true && age< 30 {
        println!("Your incentive is N1,300,000");
    } else {
        println!("Your incentive is N100,000");
    }
}