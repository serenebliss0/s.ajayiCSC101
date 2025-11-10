fn main()
{

    let arr1:[i32;4] = [10,20,30,40];
    println!("Arrays with data types");
    println!("Array is {:?}", arr1);
    println!("The size of the array is {}", arr1.len());

      let arr2 = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("Arrays without data types");
    println!("Array is {:?}", arr2);
    println!("The size of the array is {}", arr2.len());

      let arr3:[i32;8] = [01;8];
    println!("Arrays with default values");
    println!("Array is {:?}", arr3);
    println!("The size of the array is {}", arr3.len());
}