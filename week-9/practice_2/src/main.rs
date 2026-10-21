use std::io::Read;

fn main(){
//File has a  data type of struct so file is also a data type struct 
let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print!("{}", contents);

}