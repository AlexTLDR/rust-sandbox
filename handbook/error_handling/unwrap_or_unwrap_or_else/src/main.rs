fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
fn main() {
    let err_result = divide(10.0, 0.0);
    let value_or_default = err_result.unwrap_or(0.0);
    println!("Value or default: {}", value_or_default);

    // -----------------------------------------------------------------

    let none_option: Option<i32> = None;
    let option_value = none_option.unwrap_or(100);
    println!("Option or default: {}", option_value);

    // -----------------------------------------------------------------
    // unwrap_or_else(closure): Similar to unwrap_or, but if the value is Err/None,
    // it executes the provided closure (anonymous function) and returns the result of that closure.
    // This is useful if the default value requires computation:

    let err_result = divide(10.0, 0.0); // Err(...)
    let value_or_computed = err_result.unwrap_or_else(|err_msg| {
        println!("Error during division: {}. Using fallback value.", err_msg);
        -1.0 // Value computed/returned by the closure
    });
    println!("Value or computed: {}", value_or_computed);
}
