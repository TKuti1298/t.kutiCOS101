use std::io::Write;

fn main(){
    let header = "Commissioner, Ministry, Zone\n" ; 
    let commissioner: Vec<&str> = vec!["Aigbogun Alamaba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry: Vec<&str> = vec!["Internal Affairs", "Justice","Defense","Power & Steel", "Petroleum"];
    let zone:Vec<&str>= vec!["South West", "North East", "South South", "South West", "South East"];

for i in 0..commissioner.len() {
    println!("{}, {}, {}", commissioner[i], ministry[i], zone[i]);
}
    let mut file = std::fs::File::create("EFCC_MERGE.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("write header failed");

for i in 0..commissioner.len() {
    file.write_all(commissioner[i].as_bytes()).expect("write commissioner name failed");
    file.write_all(",".as_bytes()).expect("write comma failed");
    file.write_all(ministry[i].as_bytes()).expect("write ministry failed");
    file.write_all(",".as_bytes()).expect("write comma failed");
    file.write_all(zone[i].as_bytes()).expect("write zone failed");
    file.write_all("\n".as_bytes()).expect("write new line failed");
}
println!("\nData Successfully written to csv file.");
}

