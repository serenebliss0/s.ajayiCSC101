
struct Employee
{
    ceo:String,
    company:String,
    age:u32
}

fn display(emp:Employee)
{
    println!("Name is {}, company is {}, age is {}", emp.ceo, emp.company,emp.age);
}

fn main()
{
    let emp1 = Employee
    {
        company:String::from("Google Inc."),
        ceo:String::from("Sundar Pichai"),
        age:51
    };

    
    let emp2 = Employee
    {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };

    //pass emp1 and emp2 to display()

    display(emp1);
    display(emp2);
}
