fn main()
{
    let v = vec![1,2,3,4,5,6,7,8];
    let x = vec![5,6,7,8,9,10,11];
    //lets add these two vectors element by element
    for index in 0..6
    {
        let sum = v[index] + x[index];
        println!("The sum of {} and {} is {}", v[index], x[index], sum);
    }
    //ps, if it wasn't obvious, they need to have the same data type!
}