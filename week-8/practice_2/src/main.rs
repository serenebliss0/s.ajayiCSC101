fn main()
{
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut input_1 = String::new();
    println!("Enter an index to access the vector element (0-7): ");
    std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let index:usize = input_1.trim().parse().expect("Invalid input bro");

    let ch:char = v[index];

    println!("{} is the character at index location [{}]", ch, index);
}