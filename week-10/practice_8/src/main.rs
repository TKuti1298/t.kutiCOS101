struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc"),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    println!("Satya Nadella Infromation:\n");
    display(emp1);
    println!("\nSundai Pichai Infromation:\n");
    display(emp2);
}

fn display(emp:Employee) {
    println!("Name is : {}\nComapny is {}\nAge is {}"
,emp.ceo,emp.company,emp.age);
}