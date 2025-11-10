fn main()
{
    let city_arr:[&str;5] = ["Abuja", "PortHarcourt", "Maiduguri", "Kano", "Lagos"];
    println!("Array is {:?}", city_arr);
    println!("Array size is {}", city_arr.len());

    for index in 0..5
    {
        println!("City index {} is located at {}", index, city_arr[index]);
    }
}