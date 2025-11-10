//simple calculator (but with shapes)
use std::io;

pub fn trapezium(height:f64, base1:f64, base2:f64) -> f64
{
    let area:f64 = height / (2.00 * (base1 + base2));
    return area;
    
}

pub fn rhombus(diagonal1:f64, diagonal2:f64) -> f64
{
    let area:f64 = 0.5 * diagonal1 *diagonal2;
    return area;
    
}

pub fn parallelogram(base:f64, altitude:f64) -> f64
{
    let area:f64 = base * altitude;
    return area;
}

pub fn cube(length:f64) -> f64
{
    let area:f64 = 6.00 * length.powf(2.00);
    return area;
}

pub fn cylinder_volume(radius:f64, height:f64) -> f64
{
    let volume:f64 = 3.14159 * (radius.powf(2.00)) * height;
    return volume;
}


fn main()
{
    println!("Welcome user!");
    println!("Please choose an operation (1-5) to perform!");

    println!("Find the\n1. Area of a Trapezium
                                        \n2. Area of a Rhombus
                                        \n3. Area of a parallelogram
                                        \n4. Area of a cube
                                        \n5. Volume of a Cylinder\n");


    let mut  input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let mut choice:u32 = input_1.trim().parse().expect("Please type a number");

    if choice == 1
    {
        println!("You chose to find the area of a trapezium");

        println!("Please enter the height:");
        let mut height_input = String::new();
        io::stdin().read_line(&mut height_input).expect("Failed to read line");
        let height:f64 = height_input.trim().parse().expect("Please type a number");

        println!("Please enter the length of base 1:");
        let mut base1_input = String::new();
        io::stdin().read_line(&mut base1_input).expect("Failed to read line");
        let base1 = base1_input.trim().parse().expect("Enter a valid number");

        println!("Please enter the length of base 2:");
        let mut base2_input = String::new();
        io::stdin().read_line(&mut base2_input).expect("Failed to read line");
        let base2 = base2_input.trim().parse().expect("Enter a valid number");
        
        println!("The area of the trapezium is {:.2}m^2 to two decimal places", trapezium(height, base1, base2));
    }
    else if choice == 2
    {
        println!("You chose to find the area of a rhombus");

        println!("Please enter the length of the first diagonal in metres:");
        let mut diagonal1_input = String::new();
        io::stdin().read_line(&mut diagonal1_input).expect("Failed to read line");
        let diagonal1:f64 = diagonal1_input.trim().parse().expect("Please type a number");

        println!("Now enter the length of the second diagonal:");
        let mut diagonal2_input = String::new();
        io::stdin().read_line(&mut diagonal2_input).expect("Failed to read line");
        let diagonal2 = diagonal2_input.trim().parse().expect("Enter a valid number");

        println!("The area of the rhombus is {:.2}m^2 to two decimal places", rhombus(diagonal1, diagonal2));
    }
    else if choice == 3
    {
        println!("You chose to find the area of a parallelogram");

        println!("Please enter the length of the base of the parallelogram in metres:");
        let mut base_input = String::new();
        io::stdin().read_line(&mut base_input).expect("Failed to read line");
        let base:f64 = base_input.trim().parse().expect("Please type a number");

        println!("Now enter the height/altitude of the parallelogram:");
        let mut height_input = String::new();
        io::stdin().read_line(&mut height_input).expect("Failed to read line");
        let height= height_input.trim().parse().expect("Enter a valid number");

        println!("The area of the parallelogram is {:.2}m^2 to two decimal places", parallelogram(base, height));
    }
    else if choice == 4
    {
        println!("You chose to find the area of a cube");

        println!("Please enter the length of the cube in metres:");
        let mut length_input = String::new();
        io::stdin().read_line(&mut length_input).expect("Failed to read line");
        let length:f64 = length_input.trim().parse().expect("Please type a number");

        println!("The area of the cube is {:.2}m^2 to two decimal places", cube(length));
    }
    else if choice == 5
    {
        println!("You chose to find the volume of a cylinder");

        println!("Please enter the radius of the cylinder in metres:");
        let mut radius_input = String::new();
        io::stdin().read_line(&mut radius_input).expect("Failed to read line");
        let radius:f64 = radius_input.trim().parse().expect("Please type a number");

        println!("Now enter the height of the cylinder:");
        let mut height_input = String::new();
        io::stdin().read_line(&mut height_input).expect("Failed to read line");
        let height = height_input.trim().parse().expect("Enter a valid number");

        println!("The volume of the cylinder is {:.2}m^3 to two decimal places", cylinder_volume(radius, height));
    }
    else
    {
        println!("Please choose a valid option!");
        println!("Goodbye");
    }
}
