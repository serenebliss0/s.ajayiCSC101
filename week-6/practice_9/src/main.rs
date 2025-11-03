fn main()
{
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A is {}",a);
    println!("Value of B is {}",b );

    let mut res = a > b;
    println!("Is A greater than B? It is {}", res);

     let mut res = a < b;
    println!("Is A less than B? It is {}", res);

     let mut res = a >= b;
    println!("Is A greater than or equal to B? It is {}", res);

     let mut res = a <= b;
    println!("Is A less than or equal to B? It is {}", res);

     let mut res = a == b;
    println!("Is A equal to B? It is {}", res);

     let mut res = a != b;
    println!("Is A not equal to B? It is {}", res);

    
}