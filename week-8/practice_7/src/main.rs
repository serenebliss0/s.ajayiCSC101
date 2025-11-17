fn main()
{
    //explicityly defining data types in a tuple
    let datatype_tuple:(&str, f32, u8) = ("Rust", 3.14, 100);
    println!("The contents of the tuple are {:?}", datatype_tuple);

    //without defining data types in a tuple
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("The contents of the tuple are: {:?}", no_datatype_tuple);

    println!("Value at index 0 is {}", datatype_tuple.0);
    println!("Value at index 1 is {}", datatype_tuple.1);
    println!("Value at index 2 is {}", datatype_tuple.2);
}