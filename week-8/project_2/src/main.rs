use std::io::Write;
use std::io;

fn main(){

    let student_names = vec!["Oluchi Mordi\n", "Adams Aliyu\n", "Shania Bolade\n", "Adekunle Gold\n", "Blanca Edemoh\n"];
    let matric_no = vec!["ACC10211111\n", "ECO10110101\n", "CSC10328828\n", "EEE11020202\n", "MEE10202001\n"];
    let department = vec!["Accounting\n", "Economics\n", "Computer\n", "Electrical\n", "Mechanical\n"];
    let level = vec!["300\n","100\n","200\n","200\n","100\n"];


    let mut sims_file = std::fs::File::create("PAU-SIMS.xls").expect("create failed");
    sims_file.write_all("PAU SIMS\n".as_bytes()).expect("Failed to write");

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
}