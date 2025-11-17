fn main()
{
    let mut colors = ["Red", "green", "Yellow", "White"];

    println!("Original array is {:?}", colors);

    //mutable slices
    let sliced_colors = &mut colors[1..3];
    println!("Sliced array is {:?}", sliced_colors);

    //changing the value of original slice at index 1
    sliced_colors[1] = "Purple";
    println!("Modified sliced array is {:?}", sliced_colors);


}