// Program to find the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("To find the roots of the equation ax^2 + bx + c = 0");
    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let mut dis:f32 = (b.powf(2.0)) - (4.0*a*c);
    dis = dis.sqrt();


    if dis > 0.0{
        let x1 = (-b + dis) / (2.0*a);
        let x2 = (-b - dis) / (2.0*a);
        println!("The roots of the equation are: x1 = {} and x2 ={}", x1, x2);
    } else if dis == 0.0 {
        let x = -b/(2.0*a);
        println!("The root of the equation is {}", x);
    } else {
        println!("There are no real roots");
    }
}