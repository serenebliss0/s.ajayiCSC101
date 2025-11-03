use std::io;

fn main() 
{
    // Display the menu
    let items = [
        "Poundo Yam/ Edinkaiko Soup",
        "Fried Rice and Chicken",
        "Amala and Ewedu Soup",
        "Eba and Egusi Soup",
        "White Rice and Stew"
    ];
    let item_codes = ["p", "f", "a", "e", "w"];
    let prices = [3200, 3000, 2500, 2000, 2500];

    println!("Welcome, User!");
    println!("What would you like to order today?\n");

    // Print menu
    for i in 0..items.len() {
        println!("{} (Code: {}) costs ₦{}", items[i], item_codes[i], prices[i]);
    }

    // Take order code
    println!("\nPlease enter the item code of what you would like to buy:");
    let mut input_code = String::new();
    io::stdin().read_line(&mut input_code).expect("Failed to read input");
    let user_choice = input_code.trim().to_lowercase();

    // Find matching item
    if let Some(index) = item_codes.iter().position(|&c| c == user_choice.as_str()) {
        println!("You chose {}", items[index]);
        println!("Price: ₦{}", prices[index]);

        // Quantity input
        println!("Good! Now enter the quantity of {} you'd like to purchase:", items[index]);
        let mut quantity_str = String::new();
        io::stdin().read_line(&mut quantity_str).expect("Failed to read quantity");
        let quantity: i32 = quantity_str.trim().parse().expect("Please enter a valid number");

        // Calculate total
        let mut total = prices[index] * quantity;

        // Apply discount if total > 10,000
        if total > 10000 {
            let discount = (total as f32) * 0.05;
            total -= discount as i32;
            println!("A 5% discount has been applied!");
        }

        println!("Total charges: ₦{}", total);
    } else {
        println!("Food code not found");
    }
}
