fn main()
{
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ada", "Semire", "Kimi", "Esther"];

    let age = vec![16,17,19,22,20,21,18,23];

    println!("Age allocation");

    for i in 0..age.len()
    {
        println!("{} is {} years old", name[i], age[i]);
    }
}