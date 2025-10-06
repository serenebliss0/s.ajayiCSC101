fn main()
{

    let toshiba = 450_000.00 * 2.00; //price of toshiba laptops
    let mac = 1_500_000.00 * 1.00; //price of mac laptops
    let hp = 750_000.00 * 3.00; //price of hp laptops
    let dell = 2_850_000.00 * 3.00; //price of dell laptops
    let acer = 250_000.00 * 1.00; //price of acer laptops

    let sum = toshiba + mac + hp + dell + acer; //total price of laptops
    let average = sum / 10.00; //average price of laptops

    println!("The total sum of the laptops is ₦{:.2}", sum);
    println!("The average price of the laptops is ₦{:.2}", average);
}