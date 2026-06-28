// unwrap(): If Result is Ok(value) or Option is Some(value),
// it returns value. If it’s Err or None, the program will panic (crashing the current thread):
// A simple function that returns a Result, for context.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
fn main() {
    // --- Success Case ---
    let ok_result = divide(10.0, 5.0);

    // Print the Result before unwrapping to show its Ok(value) state.
    println!("The Result before unwrap: {:?}", ok_result); // Prints: Ok(2.0)

    // .unwrap() extracts the value from the Ok variant.
    let value = ok_result.unwrap();
    println!("The value after unwrap: {}", value); // Prints: 2.0
    // --- Panic Case ---
    // let err_result = divide(10.0, 0.0); // This would be Err("Division by zero")
    // println!("The Result before panic: {:?}", err_result);
    //
    // // Calling .unwrap() on an Err variant will cause the program to panic.
    // let value_panic = err_result.unwrap(); // This line would PANIC!

    // expect(“error message”): Works like unwrap, but if it panics,
    // it will use the provided message as part of the panic output.
    // It’s slightly better than unwrap as it lets you specify why you expected a value:

    let non_existent_file: Result<String, std::io::Error> =
        std::fs::read_to_string("file_no_exist.txt");
    let contents = non_existent_file.expect("Expected to find the file");
    // This will panic with the specified message
    // non_existent_file.expect("Expected the file to definitely exist!");
}
