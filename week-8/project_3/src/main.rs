use std::io::Write;
use std::io::Read;
use std::io;

fn main() {

    println!("Hello!");
    println!("Merging datasets 'Commisioner Names.txt', 'Ministry.txt', 'Geopolitical Zones.txt' into 'Convicted Ministers.txt'");
    println!("Please wait...");

//reading the existing datasets
    let mut file1 = std::fs::File::open("Commisioner Names.txt").unwrap();
    let mut contents1 = String::new();
    file1.read_to_string(&mut contents1).unwrap();


    let mut file2 = std::fs::File::open("Geopolitical Zones.txt").unwrap();
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2).unwrap();


    let mut file3 = std::fs::File::open("Ministry.txt").unwrap();
    let mut contents3 = String::new();
    file3.read_to_string(&mut contents3).unwrap();

    //creating a new file & writing the others into it

    let mut new_file = std::fs::File::create("Convicted Ministers.txt").unwrap();
    new_file.write_all(contents1.as_bytes()).expect("Failed to write");
    new_file.write_all(contents3.as_bytes()).expect("Failed to write");
    new_file.write_all(contents2.as_bytes()).expect("Failed to write");


    println!("Process Successful!");
}