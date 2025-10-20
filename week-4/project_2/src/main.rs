use std::io;

fn main()
{
    //determine if the employee is experienced or not
    println!("Please choose true, (t), (1) or false, (f), (0) for employee experience");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Please enter true or false");
    let experience_status:String = experience.trim().parse().expect("Enter a valid status");

    //collects employee age
    println!("Please enter the age of the employee");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Please enter a valid age");
    let age:u16 = age.trim().parse().expect("Enter a valid age");

    //decide which incentive is suitable for an employee based on age and experience
    let mut annual_incentive:f64 = 0.00;

    if experience_status == "true" || experience_status == "1" || experience_status == "t" || experience_status == "TRUE" && age >= 40
    {
        annual_incentive = 1_560_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if experience_status == "true" || experience_status == "1" || experience_status == "t" || experience_status == "TRUE" && age >= 30 && age < 40
    {
        annual_incentive = 1_480_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if experience_status == "true" || experience_status == "1" || experience_status == "t" || experience_status == "TRUE" && age < 28
    {
        annual_incentive = 1_300_000.00;
        println!("The annual incentive for an employee with experience status {}, aged {}, is {:.2}", experience_status, age, annual_incentive);
    }

    else if experience_status == "false" || experience_status == "0" || experience_status == "f" || experience_status == "FALSE"
    {
        annual_incentive = 100_000.00;
        println!("The annual incentive for an employee with experience status {}, is {:.2}", experience_status, annual_incentive);
    }
    
    else
    {
        println!("The information provided does not meet any criteria. \n Please try again");
        return;
    }

    if age == 0
    {
        println!("An employee cannot be working before birth!")
    }
}