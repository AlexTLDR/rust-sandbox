use std::num::ParseFloatError;
use std::{fs, io};

#[derive(Debug)]
enum ReadDivideError {
    Io(io::Error),
    Format(ParseFloatError),
    Math(String),
}

impl From<io::Error> for ReadDivideError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseFloatError> for ReadDivideError {
    fn from(err: ParseFloatError) -> Self {
        Self::Format(err)
    }
}

impl From<String> for ReadDivideError {
    fn from(err: String) -> Self {
        Self::Math(err)
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(numerator / denominator)
    }
}

// Verbose example without '?'
fn read_then_divide(file_path: &str, divisor: f64) -> Result<f64, String> {
    let content_result = std::fs::read_to_string(file_path);
    let content = match content_result {
        Ok(c) => c,
        Err(e) => return Err(format!("File read error: {}", e)), // Manual propagation
    };
    let number_result = content.trim().parse::<f64>();
    let number = match number_result {
        Ok(n) => n,
        Err(e) => return Err(format!("Number parse error: {}", e)), // Manual propagation
    };
    match divide(number, divisor) {
        // Using our previous 'divide' function
        Ok(result) => Ok(result),
        Err(e) => Err(e), // Manual propagation (simpler here)
    }
}

// Function using '?' - Note it now returns our custom error type
fn read_then_divide_with_qmark(file_path: &str, divisor: f64) -> Result<f64, ReadDivideError> {
    // '?' handles io::Error, converting it via From into ReadDivideError::Io
    let content = fs::read_to_string(file_path)?;
    // '?' handles ParseFloatError, converting it via From into ReadDivideError::Format
    let number = content.trim().parse::<f64>()?;
    // '?' handles the String error from divide, converting via From into ReadDivideError::Math
    let result = divide(number, divisor)?;
    Ok(result) // If all Ok, returns Ok(...)
}

fn main() {
    // Create a dummy file for testing
    fs::write("number.txt", "100.5").expect("Cannot write file");

    println!("Below are the read_than_divide function calls");
    match read_then_divide("number.txt", 2.0) {
        Ok(r) => println!("Result without '?': {}", r),
        Err(e) => println!("Error without '?': {:?}", e),
    }
    match read_then_divide("number.txt", 0.0) {
        Ok(r) => println!("Result without '?': {}", r),
        Err(e) => println!("Error without '?': {:?}", e),
    }
    match read_then_divide("non_existent_file.txt", 2.0) {
        Ok(r) => println!("Result without '?': {}", r),
        Err(e) => println!("Error without '?': {:?}", e),
    }

    println!("Below are the read_than_divide_with_qmark function calls");
    match read_then_divide_with_qmark("number.txt", 2.0) {
        Ok(r) => println!("Result with '?': {}", r),
        Err(e) => println!("Error with '?': {:?}", e),
    }
    match read_then_divide_with_qmark("number.txt", 0.0) {
        Ok(r) => println!("Result with '?': {}", r),
        Err(e) => println!("Error with '?': {:?}", e),
    }
    match read_then_divide_with_qmark("non_existent_file.txt", 2.0) {
        Ok(r) => println!("Result with '?': {}", r),
        Err(e) => println!("Error with '?': {:?}", e),
    }
}
