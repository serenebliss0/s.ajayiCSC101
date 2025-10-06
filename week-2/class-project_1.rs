fn main() {
    let mortgage_loan: f64 = 520_000_000.00;  // sets the mortgage loan
    let compound_interest_rate: f64 = 10.00; // sets the compound interest rate
    let time_in_years: f64 = 5.00; // sets the time in years

    let amount = mortgage_loan * (1.0 + (compound_interest_rate / 100.0)).powf(time_in_years); // calculates the amount

    let compound_interest = amount - mortgage_loan; // calculates the compound interest

    println!("The compound interest is ₦{:.2}", compound_interest); // prints the compound interest
}