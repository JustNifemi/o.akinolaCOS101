use std::io;

fn main() {
    println!("Welcome to The Public Service APS Level Checker");


    // pre-defined vectors
    let office_administrator = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["Nil","Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement Teacher", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // number of entries
    let mut input1 = String::new();

    println!("How many entries would you like to make?");
    io::stdin().read_line(&mut input1).expect("Invalid Response");
    let entry_num:u32 = input1.trim().parse().expect("Invalid Response");


// loop for vector input
let mut staff_info:Vec<(String,String,f32)> = Vec::new();
    for x in 0..entry_num{
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();


        println!("Enter staff name:");
        io::stdin().read_line(&mut input2).expect("Invalid Response");
        let name:String = input2.trim().parse().expect("Invalid Response");

        println!("Enter profession (Office Administrator/Academic/Lawyer/Teacher):");
        io::stdin().read_line(&mut input3).expect("Invalid Response");
        let prof:String = input3.trim().parse().expect("Invalid Response");

        println!("Enter years of experience:");
        io::stdin().read_line(&mut input4).expect("Invalid Response");
        let years:f32 = input4.trim().parse().expect("Invalid Response");

        staff_info.push((name,prof.to_uppercase(),years));
    }


    let mut index:usize = 0;
    while index < entry_num.try_into().unwrap(){
    println!("{} is a/an:",staff_info[index].0);
    if staff_info[index].1 == "LAWYER" && staff_info[index].2 >0.0 && staff_info[index].2 <=2.0{
        println!("{} with position APS 1-2", lawyer[0]);
    } else if staff_info[index].1 == "LAWYER" && staff_info[index].2 >2.0 && staff_info[index].2 <=5.0 {
        println!("{} with position APS 3-5",lawyer[1]);
    } else if staff_info[index].1 == "LAWYER" && staff_info[index].2 >5.0 && staff_info[index].2 <=8.0{
        println!("{} with position APS 5-8", lawyer[2]);
    } else if staff_info[index].1 == "LAWYER" && staff_info[index].2 >8.0 && staff_info[index].2 <=10.0{
        println!("{} with position EL1 8-10",lawyer[3]);
    } else if staff_info[index].1 == "LAWYER" && staff_info[index].2 >10.0 && staff_info[index].2 <=13.0{
        println!("{} with position EL2 10-13",lawyer[4]);
    } else if staff_info[index].1 == "LAWYER" && staff_info[index].2 >13.0{
        println!("{} with position SES",lawyer[5]);
    } else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >0.0 && staff_info[index].2 <=2.0{
        println!("{} with position APS 1-2",office_administrator[0]);
    }  else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >2.0 && staff_info[index].2 <=5.0{
        println!("{} with position APS 3-5",office_administrator[1]);
    }  else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >5.0 && staff_info[index].2 <=8.0{
        println!("{} with position APS 5-8",office_administrator[2]);
    }  else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >8.0 && staff_info[index].2 <=10.0{
        println!("{} with position EL1 8-10",office_administrator[3]);
    }  else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >10.0 && staff_info[index].2 <=13.0{
        println!("{} with position EL2 10-13",office_administrator[4]);
    }  else if staff_info[index].1 == "OFFICE ADMINISTRATOR" && staff_info[index].2 >13.0{
        println!("{} with position SES",office_administrator[5]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >0.0 && staff_info[index].2 <=2.0{
        println!("{} with position APS 1-2",academic[0]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >2.0 && staff_info[index].2 <=5.0{
        println!("{} with position APS 3-5",academic[1]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >5.0 && staff_info[index].2 <=8.0{
        println!("{} with position APS 5-8",academic[2]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >8.0 && staff_info[index].2 <=10.0{
        println!("{} with position EL1 8-10",academic[3]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >10.0 && staff_info[index].2 <=13.0{
        println!("{} with position EL2 10-13",academic[4]);
    }  else if staff_info[index].1 == "ACADEMIC" && staff_info[index].2 >13.0{
        println!("{} with position SES",academic[5]);
    }  else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >0.0 && staff_info[index].2 <=2.0{
        println!("{} with position APS 1-2",teacher[0]);
    } else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >2.0 && staff_info[index].2 <=5.0{
        println!("{} with position APS 3-5",teacher[1]);
    } else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >5.0 && staff_info[index].2 <=8.0{
        println!("{} with position APS 5-8",teacher[2]);
    } else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >8.0 && staff_info[index].2 <=10.0{
        println!("{} with position EL1 8-10",teacher[3]);
    } else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >10.0 && staff_info[index].2 <=13.0{
        println!("{} with position EL2 10-13",teacher[4]);
    } else if staff_info[index].1 == "TEACHER" && staff_info[index].2 >13.0{
        println!("{} with position SES",teacher[5]);
    } else{
        println!("Oops. You're not on my spectrum. Better luck next time <3.");
    }
    index+=1;
    }
   
   println!("Thank you for using The Public Service APS Level Checker");
    
}