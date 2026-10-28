struct Laptop {
    brand: String,
    price: i32
}

impl Laptop {

    fn final_price_ot(&self) -> i32 {
    self.price * 3
}
}

fn main() {

    let hp = Laptop{
        price:650000,
        brand: String::from("HP")
    } ;

    let ibm = Laptop{
        price:755000,
        brand: String::from("IBM")
    } ;

    let toshiba = Laptop{
        price:550000,
        brand: String::from("Toshiba")
    } ;

    let dell = Laptop{
        price:850000,
        brand: String::from("Dell")
    } ;

println!("3 {} laptops will cost N{}",hp.brand,hp.final_price_ot());
println!("3 {} laptops will cost N{}",ibm.brand,ibm.final_price_ot());
println!("3 {} laptops will cost N{}",toshiba.brand,toshiba.final_price_ot());
println!("3 {} laptops will cost N{}",dell.brand,dell.final_price_ot());

//If u add brand to strut then use it if not u get missing error
let total = hp.final_price_ot()+ ibm.final_price_ot() + toshiba.final_price_ot() + dell.final_price_ot(); 
println!("\nThe toal for all 12 laptops will be N{}",total);
println!("\nTo purchase, please transfer to 9036007215 Opay.\nThank you for purchasing!");
}
