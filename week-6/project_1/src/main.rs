use std::io;

// creating a function for each formula
fn trapizium(){
    // defining variables
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Please input the value of the height:");
    io::stdin().read_line(&mut height).expect("Invalid string");
    let h:f32 = height.trim().parse().expect("Invalid value");

    println!("Please input the value for base1:");
    io::stdin().read_line(&mut base1).expect("Invalid string");
    let b1:f32 = base1.trim().parse().expect("Invalid value");

    println!("Please input the value for base2:");
    io::stdin().read_line(&mut base2).expect("Invalid string");
    let b2:f32 = base2.trim().parse().expect("Invalid value");
 
 // calulating the area
    let area:f32 = h/(2.0*(b1+b2));
    println!("Area of Trapezium = height/2*(base1+base2)");
    println!("Area of the trapizium = {}",area );
}

fn rhombus(){
    // defining variables
    let mut dia1 = String::new();
    let mut dia2 = String::new();

    println!("Please input the value for diagonal1:");
    io::stdin().read_line(&mut dia1).expect("Invalid string");
    let dia1:f32 = dia1.trim().parse().expect("Invalid value");

    println!("Please input the value for diagonal2:");
    io::stdin().read_line(&mut dia2).expect("Invalid string");
    let dia2:f32 = dia2.trim().parse().expect("Invalid value");
 
 //calculating the area
    let area = (1.0/2.0)*dia1*dia2;
    println!("Area of Rhombus = 1/2 x diagonal1 x diagonal2");
    println!("Area of Rhombus = {}",area);
}

fn parallelogram(){
    // defining variables 
    let mut base = String::new();
    let mut alt = String::new();

    println!("Please input the value of the base:");
    io::stdin().read_line(&mut base).expect("Invalid string");
    let base:f32 = base.trim().parse().expect("Invalid value");

    println!("Please input the value of the altitude:");
    io::stdin().read_line(&mut alt).expect("Invalid string");
    let alt:f32 = alt.trim().parse().expect("Invalid value");

//calculating the area
    let area = base*alt;
    println!("Area of Parallelogram = base x altitude");
    println!("Area of Parallelogram = {}",area);
}

fn cube(){
    // defining variables
    let mut lenght = String::new();

    println!("Please input the lenght of the sides:");
    io::stdin().read_line(&mut lenght).expect("Invalid string");
    let lenght:f32 = lenght.trim().parse().expect("Invalid value");

    //calculating the area
    let area = 6.0*lenght.powf(2.0);
    println!("Area of Cube = 6 x (lenght of side)^2");
    println!("Area of Cube = {}",area);
}

fn cylinder(){
    // defining variables
    let mut rad = String::new();
    let mut height = String::new();

    println!("Please input the value of the radius:");
    io::stdin().read_line(&mut rad).expect("Invalid string");
    let rad:f32 = rad.trim().parse().expect("Invalid value");

    println!("Please input the value of the height");
    io::stdin().read_line(&mut height).expect("Invalid string");
    let height:f32 = height.trim().parse().expect("Invalid value");

    // calculating the volume
    let vol = (22.0/7.0)*rad.powf(2.0)*height;
    println!("Volume of Cylinder = 22/7 x radius^2 x height");
    println!("Volume of Cylinder = {}",vol);
}

fn main() {
    println!("Welcome to 'The Calculator'");
    println!("This program allows you to perform one of the following calculations:");
    println!("0. None");
    println!("1. Area of Trapezium = height/2*(base1+base2)\n2. Area of Rhombus = 1/2 x diagonal1 x diagonal2");
    println!("3. Area of Parallelogram = base x altitude\n4. Area of Cube = 6 x (lenght of side)^2\n5. Volume of Cylinder = 22/7 x radius^2 x height");


loop{
    let mut input1 = String::new();

println!("");
    println!("What formula would you like to calculate? (ie. numbers 0-5)");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let num:char = input1.trim().parse().expect("Invalid value");

println!("");
    
// invoking other functions 
    if num == '1' {
        trapizium()
    }
    else if num == '2'{
        rhombus()
    }
    else if num=='3'{
        parallelogram()
    }
    else if num=='4'{
        cube()
    }
    else if num=='5'{
        cylinder()
    }
    else if num =='0' {
        println!("Thank you for using 'The Calculator'");
        break;
    }

}
}