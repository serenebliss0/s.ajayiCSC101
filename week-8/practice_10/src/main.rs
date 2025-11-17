fn main()
{
    let b:(i32,bool,f64) = (110, true, 10.9);
    print(b);
}

fn print(x:(i32,bool,f64))
{
    println!("Inside print method");

    let (age, is_male, cgpa) = x;
    println!("Age is {}, Is male is {}, CGPA is", age, is_male, cgpa);
}

//ps no argument is passed so code won't run