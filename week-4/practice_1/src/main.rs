use std::io;

fn main()
{
    println!("\n Student Information Management  System \n");

    //name input
    println!("\n Please enter your name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
    println!("Your name is {}", name.trim());

    //input age
    println!("\n Enter your age");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("Enter a valid age");

    let age:i32 = age.trim().parse().expect("Please type a valid age!");
    println!("Your age is {}", age);
}