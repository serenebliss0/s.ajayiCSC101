use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Enter first edge of triangle");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let edge1: f32 = input1.trim().parse().expect("Please type a valid number!");

    println!("Enter second edge of triangle");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let edge2: f32 = input2.trim().parse().expect("Please type a valid number!");

    println!("Enter third edge of triangle");
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let edge3: f32 = input3.trim().parse().expect("Please type a valid number!");

    let s:f32 = (edge1 + edge2 + edge3) / 2.0;
    let mut area:f32 = (s * (s - edge1) * (s - edge2) * (s - edge3));

    area = area.sqrt();

    println!("The area of the triangle is {}", area);

}
