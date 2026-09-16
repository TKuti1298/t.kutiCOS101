use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter Your Height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid string");

    if height >= 150.0 && height <= 170.0 
    {
        println!("Congrat, You are average"); 
    }
else if height > 170.0 && height <= 195.0
{
    println!("You made it");
}
else if height < 150.0 && height > 100.0
{
    println!("Give up lil bro");
}
else {
    println!("What actually are you?");
}
}
