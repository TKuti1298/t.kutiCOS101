fn main(){
	let toshiba = 450_000*2; //900000
	let mac = 1_500_000*1; //1500000
	let hp = 750_000*3; //2250000
	let dell = 2_850_000*3; //8550000
	let acer = 250_000*1; //250000
	let tq = 10.0;
//these items values are actually their amount multiplied by their quantity.

let s = toshiba + mac + hp + dell + acer; 
let a = s as f64/tq;
println!("The sum of the sales of the items from the sales report would be {}.\nWhile the average sales of items from the sales report would be {}.",s,a)

}