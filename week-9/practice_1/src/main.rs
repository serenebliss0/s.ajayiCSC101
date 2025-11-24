use std::io::Write;

fn main()
{
    let announce = "Week 9- Rust File Input and Output Practice";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("Could not create file");
    file.write_all("Welcome to Rust programming!".as_bytes()).expect("Could not write to file");
    file.write_all(format!("\n{}\n", announce).as_bytes()).expect("Could not write to file");
    file.write_all(format!("{}\n", dept).as_bytes()).expect("Could not write to file");

    println!("Data written to data.txt successfully.");
}