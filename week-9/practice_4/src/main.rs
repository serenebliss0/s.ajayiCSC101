use std::fs::OpenOptions;
use std::io::Write;

fn main()
{
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Couldn't open file");
    
    file.write_all("Hello class".as_bytes()).expect("Failed to write data to file");
    file.write_all("This is the appendage to the document".as_bytes()).expect("Failed to write data to file");

    println!("Successfully appended file!");
}