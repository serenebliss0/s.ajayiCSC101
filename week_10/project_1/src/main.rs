use std::io;

fn main()
{
    #[derive(Debug)]
    struct Laptop
    {
        name:String,
        price:f64,
        item_code:String,
        quantity: u64
    }

    let mut customer_choices = Vec::new();

    let hp = Laptop {
        name: "HP".to_string(),
        price: 650_000.0,
        item_code: "hp".to_string(),
        quantity: 10
    };

    let ibm = Laptop {
        name: "IBM".to_string(),
        price: 755_000.0,
        item_code: "ibm".to_string(),
        quantity: 6
    };

    let toshiba = Laptop
    {
        name: "Toshiba".to_string(),
        price: 550_000.0,
        item_code: "toshiba".to_string(),
        quantity: 10
    };

    let dell = Laptop
    {
        name: "Dell".to_string(),
        price: 850_000.0,
        item_code: "dell".to_string(),
        quantity: 4
    };

    
    loop
    {

    println!("Welcome user! What would you like to order from our menu?");
    
    println!("{:?}\n{:?}\n{:?}\n{:?}", hp, ibm, toshiba, dell);

    println!("Choose an item code from the menu");


    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let user_choice = user_choice.trim().to_lowercase();
    

    match user_choice.as_str()
    {
        "hp" => {
            println!("How many hp laptops are you buying?");

            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read line");
            let quantity:u64 = match quantity.trim().parse() {
                Ok(quantity) => quantity,
                Err(e) => {
                    println!("{}",e );
                    return;
                }
            };

            if quantity > hp.quantity
            {
                println!("Sorry, we don't have enough in stock ");
            }
            else
            {
                let mut index = 1;
                for mut index in 1..=quantity
                {
                    customer_choices.push("hp");
                    index+=1;
                }
                println!("HP added to cart successfully!");
            }
        },

        "ibm" => {
            println!("How many IBM laptops are you buying?");

            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read line");
            let quantity:u64 = match quantity.trim().parse() {
                Ok(quantity) => quantity,
                Err(e) => {
                    println!("{}",e );
                    return;
                }
            };

            if quantity > ibm.quantity
            {
                println!("Sorry, we don't have enough in stock ");
            }
            else
            {
                let mut index = 1;
                for mut index in 1..=quantity
                {
                    customer_choices.push("ibm");
                    index+=1;
                }
                println!("HP added to cart successfully!");
            }
        },
    
        "toshiba" => 
        {
            println!("How many TOSHIBA laptops are you buying?");
    
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read line");
            let quantity:u64 = match quantity.trim().parse() {
                Ok(quantity) => quantity,
                Err(e) => {
                    println!("{}",e );
                    return;
                }
            };
    
            if quantity > toshiba.quantity
            {
                println!("Sorry, we don't have enough in stock ");
            }
            else
            {
                let mut index = 1;
                for mut index in 1..=quantity
                {
                    customer_choices.push("toshiba");
                    index+=1;
                }
                println!("TOSHIBA added to cart successfully!");
            }
        },
        //put here
        "dell" => 
        {
            println!("How many Dell laptops are you buying?");
    
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read line");
            let quantity:u64 = match quantity.trim().parse() {
                Ok(quantity) => quantity,
                Err(e) => {
                    println!("{}",e );
                    return;
                }
            };
    
            if quantity > dell.quantity
            {
                println!("Sorry, we don't have enough in stock ");
            }
            else
            {
                let mut index = 1;
                for mut index in 1..=quantity
                {
                    customer_choices.push("dell");
                    index+=1;
                }
                println!("DELL added to cart successfully!");
            }
        },
        _ => {
            println!("Your option didn't match any records");
            return;
        }
        }

        println!("Would you like to add anything else to the cart?\nChoose y or n");

        let mut add_more_to_cart = String::new();
        io::stdin().read_line(&mut add_more_to_cart).expect("Failed to read line");
        let add_more_to_cart = add_more_to_cart.trim().to_lowercase();
    
        if add_more_to_cart != "y"
        {
            println!("In your cart, you currently have the following items:");
            println!("{:?}", customer_choices);

            //create a variable to store the quantity chosen for each laptop
            let hp_count = customer_choices.iter().filter(|&&item| item == "hp").count();
            let ibm_count = customer_choices.iter().filter(|&&item| item == "ibm").count();
            let toshiba_count = customer_choices.iter().filter(|&&item| item == "toshiba").count();
            let dell_count = customer_choices.iter().filter(|&&item| item == "dell").count();

            //calculate the total price by each computer. if quantity is 0, then total price is 0 for that brand
            let total_hp = hp_count * 650_000;
            let total_ibm = ibm_count * 755_000;
            let total_toshiba = toshiba_count * 550_000;
            let total_dell = dell_count * 850_000;

            //sum up all the total price per computer
            let total_cost = total_dell + total_toshiba + total_ibm + total_hp;

           println!("The price of all items in your cart are: {}", total_cost);
           break;
        }
        else
        {
            continue;
        }

}

    }