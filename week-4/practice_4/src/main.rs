use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let name = input1.trim();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: u32 = input2.trim().parse().expect("Please type a valid number!");

    if age >= 18
    {
        println!("Welcome to the party, {}", name);
    }
    else
    {
        println!("Oops! You are not of age to enter the party, {}", name);
    }
}