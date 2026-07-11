use thiserror::Error;

#[derive(Error, Debug)]
enum DivisionError {
    #[error("Division by zero not allowed")]
    DivisionByZero,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, DivisionError> {
    match b != 0.0 {
        true => Ok(a / b),
        false => Err(DivisionError::DivisionByZero),
    }
}

fn main() {
    match safe_divide(15.0, 5.0) {
        Ok(v) => println!("The result is {}", v),
        Err(e) => println!("Error: {}", e),
    }

    match safe_divide(27.65, 0.0) {
        Ok(v) => println!("The result is {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
