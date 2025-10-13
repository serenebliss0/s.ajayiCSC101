fn main() {
let  fees = 25_000;
println!("fees is {}", fees);

//the following code will result in an error unless the mut keyword used
fees = 35_000;
println!("fees changed is {}", fees);

}
