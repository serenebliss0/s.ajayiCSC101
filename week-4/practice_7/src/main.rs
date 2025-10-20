use std::io;

fn main()
{
    println!("Enter a number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut num: i32 = input1.trim().parse().expect("Please type a valid number");
    while num < 10
    {
        println!("The number is {}", num);
        num += 1;
    }

    println!("Outside the loop, the number is {}", input1);
}