use std::io::*;
use std::fs::*;

fn main() {
    struct Commissioner {
        name: String,
        ministry: String,
        geopolitical_zone: String,
    }

    let mut Commissioners: Vec<Commissioner> = Vec::new();

    println!("How many commissioners are you working with?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number_of_commissioners: u32 = input.trim().parse().expect("Enter a positive whole number");

    for i in 0..number_of_commissioners {
        println!("Enter name of commissioner {}", i + 1);
        let mut name_input = String::new();
        std::io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let name = name_input.trim().to_string();

        println!("Enter the ministry the commissioner belongs to {}", i + 1);
        let mut ministry_input = String::new();
        std::io::stdin().read_line(&mut ministry_input).expect("Failed to read line");
        let ministry = ministry_input.trim().to_string();

        println!("Enter the geopolitical zone {}", i + 1);
        let mut geopolitical_zone_input = String::new();
        std::io::stdin().read_line(&mut geopolitical_zone_input).expect("Failed to read line");
        let geopolitical_zone = geopolitical_zone_input.trim().to_string();

        // create the struct
        let commissioner = Commissioner {
            name,
            ministry,
            geopolitical_zone
        };

        // push into the vector
        Commissioners.push(commissioner);

        println!("Commissioner {} added successfully!", i + 1);
    }

    // print all  commissioners
    for (i, s) in Commissioners.iter().enumerate() 
    {
        println!("Commissioner {}: {} | {} | {}", i + 1, s.name, s.ministry, s.geopolitical_zone);
    }

    let mut file = OpenOptions::new().append(true).create(true).open("commissioners.txt").expect("Failed to write to file");

    for s in &Commissioners 
    {
        let line = format!("{}\t{}\t{}\n", s.name, s.ministry, s.geopolitical_zone);
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }

    println!("All commissioners have been added to commissioners.txt successfully");

}
