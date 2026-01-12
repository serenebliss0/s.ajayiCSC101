use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("staff_tb.sql")?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Convert to String safely
    let contents = String::from_utf8_lossy(&bytes);

    println!("{}", contents);

    Ok(())
}
