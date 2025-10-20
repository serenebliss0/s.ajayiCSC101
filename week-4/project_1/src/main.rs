use std::io;

//Quadratic Equation solver

fn main()
{
    //declaring variables to store coefficients
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    //taking input from user
    println!("Enter coefficient a:");
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a: f64 = a_input.trim().parse().expect("Please type a valid number");

    println!("Enter coefficient b:");
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b: f64 = b_input.trim().parse().expect("Please type a valid number");

    println!("Enter coefficient c:");
    io::stdin().read_line(&mut c_input).expect("Failed to read line");
    let c: f64 = c_input.trim().parse().expect("Please type a valid number");

    //calculating discriminant
    let discriminant = b * b - 4.0 * a * c;

    //displays the quadratic equation and its roots based on the discriminant
    if b >0.0
    {
    println!("The expression is: {}x^2 + {}x + {} = 0", a, b, c);
    //prints out the expression in the form ax^2 + bx + c
    }
    else 
    {
        println!("The expression is {}x^2 {}x + {} = 0", a, b, c);
        //changes the expression to ax^2 -bx +c 
    }

    if discriminant > 0.0 
    {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The two real and distinct roots are: {:.4} and {:.4}", root1, root2);
    } 
    
    else if discriminant == 0.0 
    {
        let root = -b / (2.0 * a);
        println!("One real root: {:.4}", root);
    } 
    
    else 
    {
        println!("No real roots");
    }
}