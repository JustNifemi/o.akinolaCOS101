use std::io;
use std::io::Write;

fn main(){

println!("Nigerian Brewery Limited: Portfolio");
    //creating vectors
    let mut lager:Vec<String> = Vec::new();
    let mut stout:Vec<String> = Vec::new();
    let mut non_alcoholic:Vec<String>  = Vec::new();


    let mut input1  = String::new();

    println!("How many drinks do you want to input?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let entry_num:u32 = input1.trim().parse().expect("Failed to read input");


    for x in 0..entry_num{
        println!("What category would you like in input in? (1/2/3/4)use
            1. Lager
            2. Stout
            3. Non-Alcoholic
            4. None");

        let mut input2 = String::new();

        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let options:u32 = input2.trim().parse().expect("Failed to read input");
        
        if options == 1 {
            let mut input3 = String::new();

            println!("Enter drink (Lager) name:");
            io::stdin().read_line(&mut input3).expect("Failed to read input");
            let drink_lager:String = input3.trim().parse().expect("Failed to read input");

            lager.push(drink_lager + "\n");
        } else if options == 2 {
            let mut input4 = String::new();

            println!("Enter drink (Stout) name:");
            io::stdin().read_line(&mut input4).expect("Failed to read input");
            let drink_stout:String = input4.trim().parse().expect("Failed to read input");

            stout.push(drink_stout + "\n");
        } else if options == 3 {
            let mut input5 = String::new();

            println!("Enter drink (Non-Alcoholic) name:");
            io::stdin().read_line(&mut input5).expect("Failed to read input");
            let drink_non_alcoholic:String = input5.trim().parse().expect("Failed to read input");

            non_alcoholic.push(drink_non_alcoholic + "\n");
        } else if options == 4 {
            break;
        }

    }
        let mut lager_file = std::fs::File::create("Lager.txt").expect("create failed");
        let mut stout_file = std::fs::File::create("Stout.txt").expect("create failed");
        let mut non_alcoholic_file = std::fs::File::create("Non-Alcoholic.txt").expect("create failed");

        for l in lager{
            lager_file.write_all(l.as_bytes()).expect("write failed");
        }
        for s in stout{
            stout_file.write_all(s.as_bytes()).expect("write failed");
        }
        for na in non_alcoholic{
            non_alcoholic_file.write_all(na.as_bytes()).expect("write failed");
        }

        println!("Drinks Inputed!");
}