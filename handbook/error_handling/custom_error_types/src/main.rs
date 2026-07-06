use std::num::ParseIntError;
use std::{error, fmt, fs};
// Error from parsing integers
// Our custom enum for data processing errors
#[derive(Debug)] // Allows printing the error with :?
enum DataProcessingError {
    FileNotFound(String),         // Contains the name of the missing file
    InvalidFormat(ParseIntError), // Contains the original parsing error
    NegativeValue(i32),           // Contains the negative value found
    IoError(std::io::Error),      // Contains a generic I/O error
}
// We can implement methods or traits for our error enum
impl DataProcessingError {
    fn user_message(&self) -> String {
        match self {
            DataProcessingError::FileNotFound(name) => {
                format!("Error: Could not find file '{}'", name)
            }
            DataProcessingError::InvalidFormat(_) => {
                String::from("Error: Invalid data format, expected an integer.")
            }
            DataProcessingError::NegativeValue(val) => {
                format!("Error: Found negative value {}, expected positive.", val)
            }
            DataProcessingError::IoError(_) => {
                String::from("Error: Problem during reading/writing.")
            }
        }
    }
}

// Implement Display for user-friendly output (e.g., with {})
impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataProcessingError::FileNotFound(name) => write!(f, "Could not find file '{}'", name),
            DataProcessingError::InvalidFormat(e) => write!(f, "Invalid data format: {}", e),
            DataProcessingError::NegativeValue(val) => {
                write!(f, "Found negative value '{}', expected positive", val)
            }
            DataProcessingError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

// Implement the Error trait
impl error::Error for DataProcessingError {
    // The 'source()' method is optional but useful. It returns the underlying
    // error that caused this error, if any.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            // If our error is InvalidFormat, the source is ParseIntError
            DataProcessingError::InvalidFormat(e) => Some(e),
            // If our error is IoError, the source is std::io::Error
            DataProcessingError::IoError(e) => Some(e),
            // Our other errors don't have a standard underlying cause
            _ => None,
        }
    }
}

fn read_positive_number(path: &str) -> Result<i32, DataProcessingError> {
    let content = fs::read_to_string(path).map_err(DataProcessingError::IoError)?;
    let number = content
        .trim()
        .parse::<i32>()
        .map_err(DataProcessingError::InvalidFormat)?;
    if number < 0 {
        Err(DataProcessingError::NegativeValue(number))
    } else {
        Ok(number)
    }
}

fn main() {
    // Test with non-existent file (simulated)
    match read_positive_number("non_existent_data.txt") {
        Ok(n) => println!("Number read: {}", n),
        Err(e) => println!("Error: {}", e), // Uses Display impl
                                            // Output: Error: I/O error: No such file or directory ...
    }
    // Test with invalid format
    fs::write("invalid_data.txt", "abc").unwrap();
    match read_positive_number("invalid_data.txt") {
        Ok(n) => println!("Number read: {}", n),
        Err(e) => println!("Error: {}", e), // Output: Error: Invalid data format: invalid digit found in string
    }
    fs::remove_file("invalid_data.txt").ok();
    // Test with negative value
    fs::write("negative_data.txt", "-10").unwrap();
    match read_positive_number("negative_data.txt") {
        Ok(n) => println!("Number read: {}", n),
        Err(e) => println!("Error: {}", e), // Output: Error: Found negative value -10, expected positive.
    }
    fs::remove_file("negative_data.txt").ok();
    // Test success
    fs::write("valid_data.txt", "123").unwrap();
    match read_positive_number("valid_data.txt") {
        Ok(n) => println!("Number read: {}", n), // Output: Number read: 123
        Err(e) => println!("Error: {}", e),
    }
    fs::remove_file("valid_data.txt").ok();
}
