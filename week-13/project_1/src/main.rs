use semire_read::Readable;

pub mod read_sql;
use read_sql::read_sql;


enum LoginStatus {
    Success,
    Fail,
}

pub fn login() -> (u8, LoginStatus)
{
    println!("Choose your DB role from below");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Vendor");
    println!("5. Customer");

    let role = u8::read();

    //define passwords for each role
    let admin = "eeprom1@";
    let project_manager = "1e2@pyd";
    let employee = "welcome2pau@1";
    let vendor = "sell2max100";


    if role == 1
    {
        println!("Enter the password for Admin");
        let user_guessed_password = String::read();

        if user_guessed_password == admin
        {
            println!("Welcome, back, Admin");
            view_database(role, LoginStatus::Success);
            return (role, LoginStatus::Success)

        }
        else
        {
            println!("Invalid password for admin!");
            return (role, LoginStatus::Fail)
        }
    }
    else if role == 2
    {
        println!("Enter the password for project manager");
        let user_guessed_password = String::read();

        if user_guessed_password == project_manager
        {
            println!("Welcome, back, project manager");
            view_database(role, LoginStatus::Success);
            return (role, LoginStatus::Success)
        }
        else
        {
            println!("Invalid password!");
            return (role, LoginStatus::Fail)
        }
    }

    else if role == 3
    {
        println!("Enter the password for Employee");
        let user_guessed_password = String::read();

        if user_guessed_password == employee
        {
            println!("Welcome, back, Employee");
            view_database(role, LoginStatus::Success);
            return (role, LoginStatus::Success)
        }
        else
        {
            println!("Invalid password!");
            return (role, LoginStatus::Fail)
        }
    }
    
    else if role == 4
    {
        println!("Enter the password for Vendor");
        let user_guessed_password = String::read();

        if user_guessed_password == vendor
        {
            println!("Welcome, back, Vendor");
            view_database(role, LoginStatus::Success);
            return (role, LoginStatus::Success)
        }
        else
        {
            println!("Invalid password!");
            return (5, LoginStatus::Fail)
        }
    }

    else
    {
        println!("Welcome customer!");
        view_database(role, LoginStatus::Success);
        return (5, LoginStatus::Success)
    }
}

pub fn view_database(role:u8, status: LoginStatus) 
{

    match status
    {
        LoginStatus::Success => {
            match role
            {
                1 => {
                    println!("Here is the structure of the entire database!");
                    read_sql("globacom_dbase.sql");
                },
                2 => {
                    println!("This is the structure of the project table!");
                    read_sql("project_tb.sql");
                },
                3 => {
                    println!("Here is the structure of the staff table!");
                    read_sql("staff_tb.sql");
                },
                4 => {
                    println!("This is the structure for the data-plan table!");
                    read_sql("dataplan_tb.sql");
                },
                5 => {
                    println!("This is the customer table!");
                    read_sql("customers_tb.sql");
                },
                _ => {
                    println!("Invalid options received!");
                }
            }
        },
        LoginStatus::Fail => {
            println!("Incorrect password for selected role!");
        }
    }
}


fn main()
{
    login();
}