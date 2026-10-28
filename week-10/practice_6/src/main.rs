fn add_one(e: &mut i32) {
    *e+= 1;
    //*e just showing that the variable e is borrowed
}
// &mut - you may borrow and change it 
fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}
