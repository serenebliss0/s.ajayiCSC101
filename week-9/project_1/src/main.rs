use csv::Writer;
use csv::Reader;
use std::io;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

/*fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let mut rdr = Reader::from_path("drinks.csv")?;

    for (i, result) in rdr.records().enumerate()
    {
        let record = result?;
        println!("Category {}: {:?}\n", i, record);
    }
    Ok(())
}
    */

    fn main()
    {

        struct Category
        {
            name: String,
            items: Vec<String>,
        }

        println!("Welcome to Nigerian Breweries!!!");
        println!("Choose an option from the list");
        println!("1. View our catalog\n2. Add a new category");

        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).expect("Failed to read line!");
        let user_choice:u8 = match user_choice.trim().parse() {
            Ok(user_choice) => user_choice,
            Err (e) => {
                println!("Failed to read line! Try again.");
                return;
            }
        };

    if user_choice == 1
    {
        display_available_drinks();
    }
    else if user_choice == 2
    {
        add_item_to_menu();
    }
    else if user_choice == 3
    {
        add_item_to_category();
    }
    else
    {
        println!("The option you provided isn't on the list. Try again");
        return;
    }

    }

    pub fn add_item_to_menu() -> std::io::Result<()>
    {

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("drinks.txt")?;

        println!("What new category do you want to add to the catalog?");
        
        let mut input_1 = String::new();
        io::stdin().read_line(&mut input_1).expect("Failed to read line");
        let category = input_1.trim();

        writeln!(file, "{} >", category);
        println!("How many drinks are you adding to category {}", category);

        let mut input_2 = String::new();
        io::stdin().read_line(&mut input_2).expect("Failed to read line!");
        let number_of_items_in_category:u8 = input_2.trim().parse().expect("Type in a positive whole number");
        
        let mut items: Vec<String> = Vec::new();

        for i in 1..=number_of_items_in_category
        {
            println!("Enter item {} for your category", i);

            let mut input_3 = String::new();
            io::stdin().read_line(&mut input_3 ).expect("Failed to read line");
            let item = input_3.trim();

            items.push(item.to_string());

            let line = format!("{} > {}", category, items.join(", "));
            write!(file, "{}\n", line)?;

        }

        Ok(())
    }

    pub fn display_available_drinks()
    {
        let mut drinks_txt = std::fs::File::open("drinks.txt").unwrap();
        let mut drinks = String::new();
        drinks_txt.read_to_string(&mut drinks).unwrap();
        println!("{}", drinks);
    }

    pub fn add_item_to_category()
    {
        println!("I dont know");
    }