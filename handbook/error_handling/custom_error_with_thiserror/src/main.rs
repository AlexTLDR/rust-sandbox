use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum DataProcessingError {
    #[error("Invalid data format in file")]
    InvalidFormat(#[from] ParseIntError),
    #[error("Value '{0}' is negative and cannot be processed")]
    NegativeValue(i32),
    #[error("I/O error when accessing file")]
    Io(#[from] io::Error),
}
/// Reads a number from a file and ensures it's not negative.
fn read_positive_number(path: &Path) -> Result<i32, DataProcessingError> {
    // The '?' operator automatically converts io::Error into
    // DataProcessingError::Io thanks to the #[from] attribute.
    let content = fs::read_to_string(path)?;
    // The '?' operator automatically converts ParseIntError into
    // DataProcessingError::InvalidFormat thanks to the #[from] attribute.
    let number = content.trim().parse::<i32>()?;
    if number < 0 {
        // We create this error variant manually.
        Err(DataProcessingError::NegativeValue(number))
    } else {
        Ok(number)
    }
}
fn main() {
    // --- Setup dummy files for demonstration ---
    fs::write("valid_number.txt", "123").unwrap();
    fs::write("invalid_format.txt", "abc").unwrap();
    fs::write("negative_number.txt", "-42").unwrap();
    // --- Test different scenarios ---
    println!("--- Test Case 1: Success ---");
    let path_valid = Path::new("valid_number.txt");
    match read_positive_number(path_valid) {
        Ok(n) => println!("Successfully read positive number: {}", n),
        Err(e) => eprintln!("An unexpected error occurred: {}", e),
    }
    println!("\n--- Test Case 2: File Not Found ---");
    let path_nonexistent = Path::new("no_such_file.txt");
    match read_positive_number(path_nonexistent) {
        Ok(n) => println!("Read number: {}", n),
        Err(e) => eprintln!("Error: {}", e), // Will print "I/O error when accessing file"
    }
    println!("\n--- Test Case 3: Invalid Format ---");
    let path_invalid_format = Path::new("invalid_format.txt");
    match read_positive_number(path_invalid_format) {
        Ok(n) => println!("Read number: {}", n),
        Err(e) => eprintln!("Error: {}", e), // Will print "Invalid data format in file"
    }
    println!("\n--- Test Case 4: Negative Value ---");
    let path_negative = Path::new("negative_number.txt");
    match read_positive_number(path_negative) {
        Ok(n) => println!("Read number: {}", n),
        Err(e) => eprintln!("Error: {}", e), // Will print "Value '-42' is negative..."
    }
    // --- Cleanup ---
    fs::remove_file("valid_number.txt").ok();
    fs::remove_file("invalid_format.txt").ok();
    fs::remove_file("negative_number.txt").ok();
}
