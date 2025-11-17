use std::io;

fn main()
{
    let mut city:Vec<String> = Vec::new();

    println!("The city vector has {} element(s) initially", city.len());

    let mut input_1 = String::new();
    println!("Enter the number of cities you want to add: ");
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let city_count:u32 = input_1.trim().parse().expect("Please enter a valid number");

    for count in 0..city_count
    {
        let mut input_2 = String::new();
        println!("Enter city name {}: ", count + 1);
        io::stdin().read_line(&mut input_2).expect("Failed to read line");
        let new_city = input_2.trim().to_string();
        city.push(new_city);
    }

    println!("Your preferred cities are:");
    let mut count = 1;

    for i in city.iter()
    {
        println!("{}. {}", count, i);
        count += 1;
    }
}