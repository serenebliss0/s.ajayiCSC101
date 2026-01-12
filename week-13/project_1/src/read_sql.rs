use std::fs::File;
use std::io::Read;

pub fn read_sql(file_to_read:&str) -> Result<(), Box<dyn std::error::Error>> 
{
    let mut cleaned_input = format!("{}", file_to_read);

    let mut file = File::open(cleaned_input)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Convert to String safely
    let contents = String::from_utf8_lossy(&bytes);

    println!("{}", contents);

    Ok(())
}
