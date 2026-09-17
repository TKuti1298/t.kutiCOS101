use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for A");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter value for B");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter value for C");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Failed to read input");

let d:f32 = (b*b)-(4.0*a*c);
println!("Discriminant is {}", d);

if d>0.0 {
    println!("There are two distinct roots");
} else if d==0.0{
    println!("There is one real roots");
} else if d<0.0 {
    println!("There are no real roots");
}


}
