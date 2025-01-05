use std::io::Write;
use std::io;

fn main(){
    println!("PAU Student Management Information System (PAU-SMIS)");

    let mut student_names:Vec<String> = Vec::new();
    let mut matric_no:Vec<String> = Vec::new();
    let mut department:Vec<String> = Vec::new();
    let mut level:Vec<String> = Vec::new();


println!("Hello! Welcome to the PAU-SMIS input program.");


// to determine the range for the loop
println!("How many student entries would you like to make?");
let mut num = String::new();
io::stdin().read_line(&mut num).expect("Failed to read input");
let num:u32 = num.trim().parse().expect("Failed to read input");


// loop for input
for x in 0..num{
    println!("Enter student's name:");
    let mut std_name = String::new();
    io::stdin().read_line(&mut std_name).expect("Failed to read input");
    let std_name:String = std_name.trim().parse().expect("Failed to read input");

    println!("Enter student's matriculation number:");
    let mut std_matric = String::new();
    io::stdin().read_line(&mut std_matric).expect("Failed to read input");
    let std_matric:String = std_matric.trim().parse().expect("Failed to read input");

    println!("Enter student's department:");
    let mut std_dep = String::new();
    io::stdin().read_line(&mut std_dep).expect("Failed to read input");
    let std_dep:String = std_dep.trim().parse().expect("Failed to read input");

    println!("Enter student's level:");
    let mut std_level = String::new();
    io::stdin().read_line(&mut std_level).expect("Failed to read input");
    let std_level:String = std_level.trim().parse().expect("Failed to read input");


    //to push to the vectors
    student_names.push(std_name + "\n");
    matric_no.push(std_matric + "\n");
    department.push(std_dep + "\n");
    level.push(std_level + "\n")

}


    let mut sims_file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
    sims_file.write_all("PAU SMIS\n".as_bytes()).expect("Failed to write");

    sims_file.write_all("Student Name\n".as_bytes()).expect("Failed to write");
    for std in student_names{
        sims_file.write_all(std.as_bytes()).expect("Failed to write");
    }

    sims_file.write_all("Matric. Number\n".as_bytes()).expect("Failed to write");
    for mtno in matric_no{
        sims_file.write_all(mtno.as_bytes()).expect("Failed to write");
    }

    sims_file.write_all("Department\n".as_bytes()).expect("Failed to write");
    for dpt in department{
        sims_file.write_all(dpt.as_bytes()).expect("Failed to write");
    }

    sims_file.write_all("Level\n".as_bytes()).expect("Failed to write");
    for lvl in level{
        sims_file.write_all(lvl.as_bytes()).expect("Failed to write");
    }

    print!("File created.");
    println!("Thank you for using the PAU-SMIS input program.");
}