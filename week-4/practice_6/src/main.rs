use std::io;

fn main()
{
    println!("Enter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let upper_bound: i32 = input2.trim().parse().expect("Please type a valid number");
    let lower_bound: i32 = input1.trim().parse().expect("Please type a valid number");

    for x in lower_bound..=upper_bound {

        println!("Count level is {}", x);
    }

    
}