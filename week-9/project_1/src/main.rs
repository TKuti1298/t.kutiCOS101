use std::io::Write; 

fn main(){
    let header = "Lager,Stout,Non-Alcoholic";
    let row_1 = "\n33 Exports,Legend,Maltina";
    let row_2 = "\nDesperados,Turbo King,Amstel Malta";
    let row_3 = "\nGoldenberg,Williams,Malta Gold";
    let row_4 = "\nGulder,  ,Fayrouz";
    let row_5 = "\nHeineken, ,";
    let row_6 = "\nStar, ,";

    let mut file = std::fs::File::create("project_1.csv").expect("create failed");
file.write_all(header.as_bytes()).expect("write header failed");
file.write_all(row_1.as_bytes()).expect("write row 1 failed");
file.write_all(row_2.as_bytes()).expect("write row 2 failed");
file.write_all(row_3.as_bytes()).expect("write row 3 failed");
file.write_all(row_4.as_bytes()).expect("write row 4 failed");
file.write_all(row_5.as_bytes()).expect("write row 5 failed");
file.write_all(row_6.as_bytes()).expect("write row 6 failed");

println!("\nData thankfully written to csv file.");

}

/* Or,
You can use let rows = vec![]
then used for i in 0..row.len( {
file.write_all(rows[i].as_bytes()).expect("write row failed");
...
})

and resume that
*/