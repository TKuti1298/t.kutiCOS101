fn main() {
    println!("-----=Ernst & Young Global Limited=-----");

    let mut input1 = String::new();
    println!("Please input your name:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name = input1.trim().to_string();

    let years: u32 = loop {
        let mut input2 = String::new();
        println!("How many years of experience do you have, {}?", name);        std::io::stdin().read_line(&mut input2).expect("Failed to read input");

        match input2.trim().parse::<u32>() {
            Ok(n) => break n,
            Err(_) => println!("Please input a valid number of years of experience."),
        }
    };
//claude gave me candidates
    println!("{} has {} years of experience.", name, years);

    let mut candidate = vec![
        ("Adaeze Okafor", 4),
        ("Tunde Bakare", 11),
        ("Chioma Nwosu", 7),
        ("Ibrahim Musa", 15),
        ("Folake Adeyemi", 3),
        ("Emeka Eze", 9),
        ("Ngozi Obi", 12),
        ("Seun Ogunleye", 6),
        ("Hauwa Bello", 8),
        ("Chinedu Okeke", 18),
        ("Yetunde Salami", 2),
        ("Kelechi Uche", 10),
        ("Bola Adebayo", 14),
        ("Amina Yusuf", 5),
        ("Obinna Nnamdi", 13),
        ("Funmi Alade", 1),
        ("Musa Danjuma", 16),
        ("Ifeoma Anene", 20),
        ("Dayo Fashola", 0),
        ("Zainab Garba", 17),
    ];

   // Made us of the gitcopilot extension to help me 
    let mut position = 1;
    for i in 0..candidate.len() {
        if candidate[i].1 > years {
            position += 1;
        }
    }

    candidate.push((&name, years));
    candidate.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Ranking of candidates based on years of experience:");
    for i in 0..candidate.len() {
        println!("{}. {} - {} years", i + 1, candidate[i].0, candidate[i].1);
    }

    if position <= 5 {
        println!("Congratulations, {}! You are in the top 5 candidates with {} years of experience.", name, years);
    } else {
        println!("Sorry, {}. You are ranked {} out of {} candidates with {} years of experience.", name, position, candidate.len(), years);
    }

    println!("Thank you for participating.\nPlease come again");
}
//pretier extension was used here
// i hate claude :(