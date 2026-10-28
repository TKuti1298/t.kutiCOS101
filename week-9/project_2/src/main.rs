use std::io::Write;

fn main(){
    let header = "Student Name,Matric Number,Department,Level\n" ; 
    let students = vec![
        ("Oluchi Mordi","ACC10211111","Accounting",300),
        ("Adams Aliyu","ECO10110101","Economics",100),
        ("Shania Bolade","CSC10328828","Computer",200),
        ("Adekunle Gold","EEE11020202","Electrical",200),
        ("Blanca Edemoh","MEE10202001","Mechanical",100)
    ];
/*could have used 0..5 but it wouldnt work if u added a new student.
 You would have to manually change it.*/ 
for i in 0..students.len() {
    println!("{}, {}, {}, {}", students[i].0, students[i].1, students[i].2, students[i].3);
}
    let mut file = std::fs::File::create("PAU_SIMS.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("write header failed");

for i in 0..students.len() {
    file.write_all(students[i].0.as_bytes()).expect("write student name failed");
    file.write_all(",".as_bytes()).expect("write comma failed");
    file.write_all(students[i].1.as_bytes()).expect("write matric number failed");
    file.write_all(",".as_bytes()).expect("write comma failed");
    file.write_all(students[i].2.as_bytes()).expect("write department failed");
    file.write_all(",".as_bytes()).expect("write comma failed");
    file.write_all(students[i].3.to_string().as_bytes()).expect("write level failed");
    file.write_all("\n".as_bytes()).expect("write new line failed");
}
println!("\nData Successfully written to csv file.");
}

