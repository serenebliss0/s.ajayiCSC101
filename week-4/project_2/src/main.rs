use std::io;

fn main()
{
    loop{
    //determine if the employee is experienced or not

    println!("Is the employee experienced? Enter true (t) or false (f)");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Please enter true or false");
    let experience_status_raw:String = experience.trim().to_lowercase().parse().expect("Enter a valid status");
    //collects the raw user input for normalization

    //collects employee age
    println!("Please enter the age of the employee");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Please enter a valid age");
    let age:u8 = age.trim().parse().expect("Enter a valid age");

            //normalize experience to true or false
           let experience_status = 
           if experience_status_raw == "true" || experience_status_raw == "1" || experience_status_raw == "t" 
           { 
            "true" // this value will be assigned to the outer experience_status
            }   
        else
         {
                "false" // same here
         };
    
    //decide which incentive is suitable for an employee based on age and experience
    let mut annual_incentive:f64 = 0.00;

       if age == 0
    {
        println!("An employee cannot be working before birth!");
        break;
    }

    if (experience_status == "true") && age >= 40
    {
        annual_incentive = 1_560_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if (experience_status == "true" && age >= 30) && age < 40
    {
        annual_incentive = 1_480_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if (experience_status == "true") && age < 28
    {
        annual_incentive = 1_300_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if experience_status == "false"
    {
        annual_incentive = 100_000.00;
        println!("The annual incentive for an employee with experience status {}, is {:.2}", experience_status, annual_incentive);
    }
    
    else
    {
        println!("The information provided does not meet any criteria. \n Please try again");
    }

    //allow the user to run the program again for multiple employees
    println!("Do you want to find the incentive for another employee?\nChoose either y or n only");

    let mut  input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Enter y or n");
    let option_1:String = input_1.trim().to_lowercase();

    if option_1 != "y"
    {
        println!("Goodbye");
        break;
    }
}
}