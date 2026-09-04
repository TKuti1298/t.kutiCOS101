fn main(){
	let toshiba:f64 = 900_000.0; //450_000*2
	let mac:f64 = 1_500_000.0; //1_500_000*1
	let hp:f64 = 2_250_000.0; //750_000*3
	let dell:f64 = 8_550_000.0; //2_850_000*3
	let acer:f64 = 250_000.0; //250_000*1
	let tq:f64 = 10.0;
//these items values are actually their amount multiplied by their quantity.

let s = toshiba + mac + hp + dell + acer; 
let a = s/tq;
println!("The sum of the sales of the items from the sales report would be {}.\nWhile the average sales of items from the sales report would be {}.",s,a)

}