use std::io;

fn main() {

let mut experience = String::new();
println!("Do you have any experienced?\n(y/n)");
io::stdin().read_line(&mut experience).expect("Failed to read input");
let experienced = experience.trim().to_lowercase() == "y"; 

let mut age = String::new();
println!("Enter age");
io::stdin().read_line(&mut age).expect("Failed to read value");
let age:i32 = age.trim().parse().expect("Failed to read value");

if experienced {
    if age >= 40 {
        println!("Your annual incentive is 1,560,000 naira");
    }
    else if age >=30 && age <=39 {
        println!("Your annual incentive is 1,480,000 naira ");
    }
    else if age < 28 {
        println!("Your annual incentive is 1,300,00 naira ");
    }
 } else {
    println!("Your annual incentive is 100,000 naira");
 }

}
