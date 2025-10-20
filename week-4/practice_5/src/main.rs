use std::io;

fn main()
{
  let mut input = String::new();

    println!("\n Enter your height in cm");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: f32 = input.trim().parse().expect("Please type a valid number");

    if height >= 1500.0 && height <= 170.0
    {
        println!("You are of average height");
    }
    else if height > 170.0 && height <= 190.0
    {
        println!("You are tall");
    }
    else if height > 190.0
    {
        println!("You are very tall");
    }
    else
    {
        println!("You are short");
    }

}
  