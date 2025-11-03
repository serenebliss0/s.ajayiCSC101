fn main()
{
    let fullname = "  Pan Atlantic University  ";
    println!("");
    println!("Name is {}", fullname);
    println!("");
    println!("Before trim");
    println!("Length is {}", fullname.len());
    println!("");
    println!("After trimming");
    println!("Length after trim is {}", fullname.trim().len());
}