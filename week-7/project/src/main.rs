use std::io;


fn values(prompt: &str) -> f64{
    loop{
println!("{}", prompt);
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input"); 

match input.trim().parse::<f64>() {
    Ok(n) => return n,
    Err (_) => println!("Enter valid number")
}
}
}
// Areas only 
fn trapezium(height: f64, base1: f64, base2: f64) -> f64 {
height / 2.0 * (base1 + base2)
}
fn rhombus(diagonal1: f64, diagonal2: f64) -> f64{
0.5 * diagonal1 * diagonal2
}
fn parallelogram(base: f64,altitude: f64) -> f64 {
base * altitude
}
fn cube(side: f64) -> f64{
6.0 * side * side
}
fn cylinder(radius: f64,height: f64) -> f64 {
22.0/7.0 *  radius * radius * height
}
fn main(){
    println!("What shape are we working with today?");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface area of a Cube");
    println!("5. Volume of a Cylinder");

    let choice = values("Enter your choice (1-5)") as u32;

    match choice {
        1 => {
            let h = values("Enter height: ");
            let b1 = values("Enter base 1: ");
            let b2 = values("Enter base 2: ");
            println!("Area = {}",trapezium(h, b1, b2));
        }
        2 => {
            let d1 = values("Enter diagonal 1: ");
            let d2 = values("Enter diagonal 2: ");
            println!("Area = {}",rhombus(d1, d2));
        }
        3 => {
            let base = values("Enter base: ");
            let altitude = values("Enter altitde: ");
            println!("Area = {}",parallelogram(base, altitude));
        }
        4 => {
            let side = values("Enter side: ");
            println!("Suface Area = {}",cube(side));
        }
        5 => {
            let radius = values("Enter radius: ");
            let height = values("Enter height: ");
            println!("Volume = {}",cylinder(radius, height));
        }
        _=> println!("Invalid choice."),
        }
    }
