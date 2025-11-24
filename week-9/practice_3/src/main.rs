use std::fs;

fn main()
{
    fs::remove_file("data.txt").expect("Couldn't remove file");
    println!("File removed successfully!");
}