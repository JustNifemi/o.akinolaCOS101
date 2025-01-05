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
    println!("Note: Please do not use commas (,) while inputing the student's data.");

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
    student_names.push(std_name);
    matric_no.push(std_matric);
    department.push(std_dep);
    level.push(std_level);

}


    let mut smis_file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
    smis_file.write_all("PAU SMIS\n".as_bytes()).expect("Failed to write");

    smis_file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("Failed to write");

//wrinting into the csv file

        let mut index = 0;
    for i in 0..student_names.len(){
        let file = format!("{},{},{},{}\n", student_names[index], matric_no[index], department[index], level[index]);
        smis_file.write_all(file.as_bytes()).expect("Failed to write");
        index+= 1
    }

    print!("File created.");
    println!("Thank you for using the PAU-SMIS input program.");
}