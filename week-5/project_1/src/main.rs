use std::io;
fn main() {
    println!("Welcome to OAK's Kitchen <3");
    
    // menu display
    println!("                 Menu                          Price
         P = Pounded Yam/Edinkaiko Soup      -N3,200
         F = Fried Rice & Chicken            -N3,000
         A = Amala & Ewedu Soup              -N2,500
         E = Eba & Egusi Soup                -N2,000
         W = White Rice & Stew               -N2,500");


// price input
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();

    println!("Please type in your order from the menu above (P/F/A/E/W)");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let order = input1.trim().to_uppercase();

    println!("Please type in the quantity in numbers");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quan:i32 = input2.trim().parse().expect("Not a valid number");


if order == "P" {
    let tot_price = 3200 * quan;
} else if order == "F" {
    let tot_price = 3000 * quan;
} else if order == "A" {
    let tot_price = 2500 * quan;
} else if order == "E" {
    let tot_price = 2000 * quan;
}else if order == "W" {
    let tot_price = 2500 * quan;
} 

if tot_price > 10000{
    let fin_price = tot_price * (5/100);
    println!("Your bill is :N{}",fin_price );
} else {
    println!("Your bill is :N{}",tot_price )
}

println!("Thank you for patronage! <3");

}
