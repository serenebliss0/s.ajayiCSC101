fn main()
{
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of the number is {}", num);
}

fn mutate_num_to_zero(mut param_num: i32)
{
    param_num = param_num * 0;
    println!("param num value is {}", param_num);
}