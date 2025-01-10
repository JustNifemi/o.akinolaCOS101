#[derive(Debug)]
struct Laptops {
   name:String,
   price:u32,
   quantity:u32
}
impl Laptops {
    fn price(&self)->u32 {
        self.price * self.quantity
    }
}

fn main(){
    let lap1 = Laptops{
        name:String::from("HP"),
        price:650000,
        quantity:3
    };

    let lap2 = Laptops{
        name:String::from("IBM"),
        price:755000,
        quantity:3
    };

    let lap3 = Laptops{
        name:String::from("Toshiba"),
        price:550000,
        quantity:3
    };

    let lap4 = Laptops{
        name:String::from("Dell"),
        price:850000,
        quantity:3
    };
    let total = lap1.price() + lap2.price() + lap3.price() + lap4.price();

    println!("The total cost of 3 HP laptops, 3 IBM laptops, 3 Toshiba laptops, and 3 Dell laptops is: {}", total);
}