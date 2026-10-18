/* I switched over to VS code instead of sublime text cause i feels way more interestig and cool.
I added the prettier extentsion to make my code look neater incase it looks funny or something.  */

fn main() {
    let _public_servant_level = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("-----=APS Level Checker=-----");

    loop {
        let mut bool_input = String::new();
        println!("Would you like to check your APS level? (y/n)");
        std::io::stdin().read_line(&mut bool_input).expect("Failed to read input");
        let answer = bool_input.trim().to_lowercase();

        match answer.as_str() {
            "y" => {
                println!("Alright! Let's get started.");
                break;
            }
            "n" => {
                println!("Thanks for stopping by");
                return;
            }
            _ => {
                println!("Invalid input. Please enter 'y' or 'n' ONLY");
            }
        }
    }

    let mut name_input = String::new();
    println!("Please enter your name:");
    std::io::stdin().read_line(&mut name_input).expect("Failed to read input");
    let name = name_input.trim().to_string();

    //AI aid was used in this section at lines 49 
    let department: usize = loop {
        let mut input1 = String::new();
        println!("Which department do you fall under?");
        println!(" 1. Office Admin");
        println!(" 2. Academic");
        println!(" 3. Lawyer");
        println!(" 4. Teacher");
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");

        match input1.trim().parse::<usize>() {
            Ok(n) => {
                if n >= 1 && n <= 4 {
                    break n;
                } else {
                    println!("Invalid input. Please enter a number between 1 and 4.\n");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.\n");
            }
        }
    };

    let _job = match department {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid department selection.");
            return;
        }
    };

    println!("\nSelect your job title:");
    for i in 0.._job.len() {
        println!("{}: {}", i + 1, _job[i]);
    }

    //Ai aid was also used here
    let choice: usize = loop {
        let mut input2 = String::new();
        println!("Enter the number assinged to your job title:");
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");

        match input2.trim().parse::<usize>() {
            Ok(n) => {
                if n >= 1 && n <= _job.len() {
                    break n;
                } else {
                    println!("Invalid input. Please enter a number between 1 and {}.", _job.len());
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and {}.", _job.len());
            }
        }
    };

    println!("\nIndividual named {} has a job position of: {}", name, _job[choice - 1]);
    println!("APS Level: {}", _public_servant_level[choice - 1]);
}