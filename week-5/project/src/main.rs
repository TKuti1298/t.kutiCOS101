use std::io;

fn main() {

    println!("Hello, welcome to Mama Nkechi's wonderous deligths.");
    println!("\n");
    println!("P Poundo Yam/Edinkaiko Soup N3,200");
    println!("F Fried Rice & Chicken N3,000");
    println!("A Amala & Ewedu Soup N2,500");
    println!("E Eba & Egusi Soup N2,000");
    println!("W white Rice & Stew N2,500");

let mut food = String::new();
println!("What would you like to eat?");
io::stdin().read_line(&mut food).expect("Failed to understand choice enter proper format");
let food= food.trim().to_uppercase();

let mut quantity = String::new();
println!("How many portions would you like?");
io::stdin().read_line(&mut quantity).expect("Failed to read input");
let quantity:u32 = quantity.trim().parse().expect("Failed to read input");

let price = match food.as_str() {
    "P" => 3200,
    "F" => 3000,
    "A" => 2500,
    "E" => 2000,
    "W" => 2500,
    _ => {
        println!("Invalid choice");
        return;
    }
};

let total:u32 =price * quantity;
println!("Your current total is N{}",total);

if total > 10000{ 
let new_total = total - (total*5/100);
println!("Congrats you were able to get a discount!");
println!("Your new total is N{}",new_total);
}
println!("\n");
println!("Thank you for comiing to Mama Nkechi's wonderous deligths.\nDo come again!");
}

