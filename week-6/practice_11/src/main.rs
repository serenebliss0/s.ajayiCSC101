fn main()
{
    let a:i32 = 2;
    let b:i32 = 3;

    let mut result:i32;

    result = a & b;
    println!("a & b is {}", result);
    
    result = a |  b;
    println!("a | b is {}", result);

    result = a ^ b;
    println!("a ^ b is {}", result);

    result = !b;
    println!("!b is {}", result);

    result = a << b;
    println!("a << b is {}", result);

    result = a >> b;
    println!("a >> b is {}", result);
    
}