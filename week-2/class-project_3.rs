fn main() {
    let price: f64 = 510_000.00; // price of the television set
    let depreciation_rate: f64 = 5.00; // depreciation rate
    let time_in_years: f64 = 3.00; // time in years

    let amount: f64 = price * (1.00 - (depreciation_rate / 100.00)).powf(time_in_years);

    println!("The depreciation after {} years is ₦{:.2}", time_in_years, amount) //prints out the depreciation value
}