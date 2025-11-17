fn main()
{
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("The original tuple is {:?}", mountain_heights);

    //changing values in a tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("The modified tuple is {:?}", mountain_heights);
}