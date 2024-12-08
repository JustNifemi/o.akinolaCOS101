use std::io;

fn main() {
    println!("EY Nigeria's Experience Checker");
    println!("Welcome HR");

    // defining the number of interviews and limit of loop
    println!("How many interview results would you like to input?");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid response");
    let entry_num:u32 = input1.trim().parse().expect("Invalid response");


// creating the vectors and pushing into them

let mut cand_info:Vec<(String,f32)> = Vec::new();
for x in 0..entry_num{
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter candidate's name:");
    io::stdin().read_line(&mut input2).expect("Invalid response");
    let name:String = input2.trim().parse().expect("Invalid response");

    println!("Enter candidate's years of experience:");
    io::stdin().read_line(&mut input3).expect("Invalid response");
    let years:f32 = input3.trim().parse().expect("Invalid response");

    cand_info.push((name,years));
}

    cand_info.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("");
    println!("Candidate list:");
    let mut list=1;
    for i in &cand_info{
        println!("{}. {} {}",list,i.0,i.1);
        list+=1
    }

    println!("");
    println!("The candidate with the highest number of years of experience is {} {}", cand_info[0].0, cand_info[0].1);
}
