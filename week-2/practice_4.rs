fn main() {
let p:f64 = 1000.0;
let r:f64 = 1.0;
let t:f64 = 2.0;

// simple interest

let a = p * (1.0 + (r / 100.0)) * t;
println!("The amount is {}", a);
let si = a - p;
println!("The simple interest is {}", si);
}