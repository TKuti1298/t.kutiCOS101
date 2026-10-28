struct Rectangle {
    width:u32, height:u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        //&self means we can use width and height in rectangle provided in struct
        self.width * self.height
    }
    fn perimeter(&self) -> u32{
        (2*self.width)+(2*self.height)
    } 
}
fn main() {
    let small = Rectangle{
        width:10,
        height:20
    };

    println!("Width is {} \nHeight is {} \nArea of rectangle is {}",small.width,small.height,small.area());
    println!("While the perimeter of the rectangle is {}",small.perimeter());
}