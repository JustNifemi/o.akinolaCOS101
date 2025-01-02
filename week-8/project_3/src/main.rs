use std::io::Write;
use std::io::Read;
use std::io;

fn main() {

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
}