fn main()
{
    let numbers = [1,2,3,4,5];
    println!("Original array is {:?}", numbers);

    let slice_1 = &numbers[1..3]; //this is a slice from index 1 to index 2
    let slice_2 = &numbers[..3]; //this is a slice from start to index 2
    let slice_3 = &numbers[2..]; //every element from index 2
    let slice_4 = &numbers[..]; //every element from start to end

    println!("2nd and 3rd elements are {:?}", slice_1);
    println!("1st to 3rd elements are {:?}", slice_2);
    println!("Index 2 to index 5 slice is {:?}", slice_3);
    println!("Complete array slice is {:?}", slice_4);
}