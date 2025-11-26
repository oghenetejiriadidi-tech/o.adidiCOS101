use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> 
    let categories = [
        "Lager",
        "Stout",
        "Non-Alcoholics",
        "Spirit",
    ];

    let mut file = File::create("nb_products.txt")?;

    // Write each item into the file
    for item in categories.iter() {
        writeln!(file, "{}", item)?;
    }

    println!("File created successfully: nb_products.txt");
    Ok(())
}
