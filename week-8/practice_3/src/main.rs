fn value(n:Option<&char>)
{
    println!("Element of vector {:?}", n);
}

fn main()
{
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'E', 'A', 'N'];

    let mut input_1 = String::new();
    println!("Enter an index to access the vector element (0-8): ");
    std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let index:usize = input_1.trim().parse().expect("Invalid input bro");

    let ch:Option<&char> = v.get(index);

    value(ch);
}