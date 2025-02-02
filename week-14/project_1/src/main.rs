use std::io;
use std::io::Read;

fn administrator (){
    let mut admin = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents_admin = String::new();
    admin.read_to_string(&mut contents_admin).unwrap();
    println!("{contents_admin}");
}

fn project_manager (){
    let mut pro_man = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents_pro_man = String::new();
    pro_man.read_to_string(&mut contents_pro_man).unwrap();
    println!("{contents_pro_man}");
}

fn employee (){
    let mut emp = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents_emp = String::new();
    emp.read_to_string(&mut contents_emp).unwrap();
    println!("{contents_emp}");
}

fn customer (){
    let mut cust = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents_cust = String::new();
    cust.read_to_string(&mut contents_cust).unwrap();
    println!("{contents_cust}");
}

fn vendor (){
    let mut ven = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents_ven = String::new();
    ven.read_to_string(&mut contents_ven).unwrap();
    println!("{contents_ven}");
}

fn main (){
    println!("Welcome to Globacom Ltd's Database Schema Retriever.");
    println!("\nPlease select your current position from the list below: \n
    1. Administrator
    2. Project Manager
    3. Employee
    4. Customer
    5. Vendor
    6. None");

    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed to read input");
    let status:u32 = status.trim().parse().expect("Failed to read input");

    if status == 1 {
        administrator()
    } else if status == 2 {
        project_manager()
    } else if status == 3 {
        employee()
    }else if status == 4 {
        customer()
    } else if status == 5 {
        vendor()
    } else if status == 6 {
        println!("We see your current position is not listed here. We will work on implementing more positions soon!");
    }

    println!("Thank you for using Globacom Ltd's Database Schema Retriever.");
}