use std::io;

fn main()
{
    println!("Welcome to EY Limited! The largest professional service network in the world!");

    //create a struct to store employee data

    struct Interviewee
    {
        name: String,
        age: u32,
        years_of_experience:u32,
    }

    let mut people: Vec<Interviewee> = Vec::new();

    println!("How many people are you interviewing today?");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let number_of_people = input_1.trim().parse().expect("Enter a positive whole number");

    for i in 0..number_of_people
    {
        println!("What is the name of interviewee number {}?", i + 1);
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let name = name_input.trim().to_string();

        println!("What is {}'s age?", name);
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");
        let age: u32 = age_input.trim().parse().expect("Enter a positive whole number");

        println!("How many years of experience does {} have?", name);
        let mut experience_input = String::new();
        io::stdin().read_line(&mut experience_input).expect("Failed to read line");
        let years_of_experience: u32 = experience_input.trim().parse().expect("Enter a positive whole number");

        let interviewee = Interviewee
        {
            name: name,
            age: age,
            years_of_experience: years_of_experience,
        };

        people.push(interviewee);

        println!("Interviewee {} added successfully!", i + 1);

    }

    people.sort_by_key(|p| std::cmp::Reverse(p.years_of_experience));
    println!("\nInterviewees sorted by years of experience (most to least):");

    for (i, p) in people.iter().enumerate() {
    println!("Rank {} => {} ({} years)", i + 1, p.name, p.years_of_experience);
}
}