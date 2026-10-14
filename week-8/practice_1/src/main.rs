fn main() { 

let v : Vec<i64> = Vec::new();
println!("\nThe length off Vec::new is {}",v.len());
//to add values to this, make sure its let mut not just let and then add v.push()

let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];
println!("\nThe length of vec macro is: { }",v.len());
}