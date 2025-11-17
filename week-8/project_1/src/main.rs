use std::io;


fn main()
{
    //create a vector containing all the job titles
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate Lawyer", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement Teacher", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    //to handle cases where a staff exceeds the maximum years of experience
    let min_years = vec![0, 3, 6, 9, 11];
    let max_years = vec![2, 5, 8 , 10, 13];
    println!("Welcome to Job Titles!");
    println!("Please choose the category your job belongs to:");
    println!("1. Office Administration");
    println!("2. Academic");
    println!("3. Law");
    println!("4. Teacher");
    
    //decide which category the staff belongs to
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option:u8 = option.trim().parse().expect("Type in the number that corresponds to your job category");

    let mut job_title_input = String::new(); // raw input for the job index
    let mut title = String::new(); // this is the corresponding title that goes with the index
    let mut job_index: usize = 0; // store the numeric index for later validation

    if option == 1
    {
        println!("Please choose your job title:");
        
        for i in 0..office_admin.len()
        {
            println!("{}. {}", i+1, office_admin[i]); //increments i after every loop
        }

        io::stdin().read_line(&mut job_title_input).expect("Failed to read line");
        job_index = job_title_input.trim().parse().expect("Type in the number that corresponds with your title");
        println!("Your job title is: {}", office_admin[job_index-1]);
        title = office_admin[job_index - 1].to_string();
    }
    else if option == 2
    {
        println!("Please choose your job title");

        for i in 0..academic.len()
        {
            println!("{}. {}", i + 1, academic[i]);
        }

        io::stdin().read_line(&mut job_title_input).expect("Failed to read line");
        job_index = job_title_input.trim().parse().expect("Enter the number that corresponds to your job title");
        println!("Your title is {}", academic[job_index -1]);
        title = academic[job_index-1].to_string();
    }
    
    else if option == 3
    {
        println!("Please choose your job title");

        for i in 0..lawyer.len()
        {
            println!("{}. {}", i + 1, lawyer[i]);
        }

        io::stdin().read_line(&mut job_title_input).expect("Failed to read line");
        job_index = job_title_input.trim().parse().expect("Enter the number that corresponds with your job title");
        println!("Your title is {}", lawyer[job_index - 1]);
        title = lawyer[job_index-1].to_string();
    }
    else if option == 4
    {
        println!("Please choose your job title");

        for i in 0..teacher.len()
        {
            println!("{}. {}", i + 1, teacher[i]);
        }

        io::stdin().read_line(&mut job_title_input).expect("Failed to read line");
        job_index = job_title_input.trim().parse().expect("Enter the number that matches your job title");
        println!("Your title is {}", teacher[job_index - 1]);
        title = teacher[job_index - 1].to_string();
    }
    else
    {
        println!("The category you provided doesn't meet any criteria");
        return; // exit early so job_index isn't 0
    }

    println!("How many years of experience do you have?");

    let mut years_of_experience = String::new();
    io::stdin().read_line(&mut years_of_experience).expect("Failed to read line");
    let years_of_experience:u8 = years_of_experience.trim().parse().expect("Enter the number of years of experience that you have");

    // Use the job_index we got from the user input earlier
    let min_exp = min_years[job_index - 1];
    let max_exp = max_years[job_index - 1];

    if years_of_experience < min_exp
    {
        println!("You don't qualify for this position");
    } 
    else if years_of_experience > max_exp 
    {
        println!("Experience exceeds this APS bracket. Consider Promotion?");
    }
    else
    {
        println!("Valid experience for {}", title);
        // proceed to give an aps value to the staff member
    }
            match years_of_experience
        {
            0..2 => println!("With position [{}] and [{}] years of experience, you are rated [APS 1-2] ", title, years_of_experience),
            3..5 => println!("With position [{}] and [{}] years of experience, you are rated [APS 3-5]", title, years_of_experience),
            5..8 => println!("With position [{}] and [{}] years of experience, you are rated [APS 5-8]", title, years_of_experience),
            8..10 => println!("With position [{}] and [{}] years of experience, you are rated [EL1 8-10]", title, years_of_experience),
            10..13=> println!("With position [{}]and [{}] years of experience, you are rated [APS 10-13]", title, years_of_experience),
            5..8 => println!("With position [{}] and [{}] years of experience, you are rated [SES]", title, years_of_experience),
            5..8 => println!("You don't meet any criteria : ("),
            _ => println!(""),
            13.. => println!(""),
        };

}