fn main(){
    let v = vec![10,20,30];
    let v2 = v ;
    display(&v2); // borrow doesnt move ownership so its still in v2 
    println!("In main {:?}", v2);
}
// the v is just a parameter dont get lost 
fn display(v:&Vec<i32>){
    println!("inside display {:?}", v)
}