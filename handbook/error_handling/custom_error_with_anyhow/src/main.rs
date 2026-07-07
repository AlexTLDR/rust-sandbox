use anyhow::{bail, Context, Result};
// Import Result, Context, bail
use std::fs;
// The function now returns anyhow::Result
fn read_positive_number_anyhow(path: &str) -> Result<i32> {
    let content = fs::read_to_string(path)
        // context() adds contextual information to the original error
        .context(format!("Failed to read file '{}'", path))?;
    let number = content
        .trim()
        .parse::<i32>()
        .context("File content is not a valid integer")?;
    if number < 0 {
        // bail! is an easy way to create and return an anyhow::Error
        bail!("The number read ({}) must be positive.", number);
    }
    Ok(number)
}
// main can also return anyhow::Result<()> for easy error propagation
fn main() -> Result<()> {
    fs::write("valid_data_ah.txt", "789").unwrap();
    let number = read_positive_number_anyhow("valid_data_ah.txt")?;
    println!("Success with anyhow: {}", number);
    fs::remove_file("valid_data_ah.txt").ok();
    // Test with error (negative value)
    fs::write("negative_data_ah.txt", "-5").unwrap();
    match read_positive_number_anyhow("negative_data_ah.txt") {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => {
            // anyhow formats the error including context and cause chain
            println!("Error with anyhow: {:?}", e);
            // Output (approx): Error: The number read (-5) must be positive.
        }
    }
    fs::remove_file("negative_data_ah.txt").ok();
    // Test with error (file not found)
    match read_positive_number_anyhow("nonexistent_ah.txt") {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => {
            println!("Error with anyhow: {:?}", e);
            // Output (approx):
            // Error: Failed to read file 'nonexistent_ah.txt'
            //
            // Caused by:
            //     No such file or directory (os error 2)
        }
    }
    Ok(()) // main returns Ok if everything succeeded
}
