use std::num::ParseIntError;

// and_then: This chains another operation that might fail.
// The and_then method is used when you want to perform a subsequent operation that also returns a Result.
// If the initial Result is Ok, and_then calls the closure with the success value.
// The closure itself must return a new Result.
// If the initial Result is Err, the closure is not called, and the error is propagated:
fn check_if_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err("Number is not even".to_string())
    }
}

fn main() {
    // map(): This applies a function to the contained Ok value.
    // The map method takes a closure and applies it to the value inside an Ok, leaving an Err value untouched.
    // The closure takes the success value (T) and returns a new value (U), and map returns Result<U, E>:

    let successful_parse: Result<i32, _> = "10".parse();
    // If successful_parse is Ok(10), the closure |x| x * 2 is called with 10.
    // The result is Ok(20).
    let doubled_result = successful_parse.map(|x| x * 2);
    println!("Doubled result: {:?}", doubled_result); //Prints: Ok(20)

    // --------------------------------------------------------------------------------

    let successful_parse: Result<i32, _> = "12".parse().map_err(|e: ParseIntError| e.to_string());
    let failed_parse: Result<i32, _> = "7".parse().map_err(|e: ParseIntError| e.to_string());
    // chain the parsing with the even check
    let even_result = successful_parse.and_then(check_if_even);
    println!("Result for '12':{:?}", even_result);
    let odd_result = failed_parse.and_then(check_if_even);
    println!("Result for '7':{:?}", odd_result);

    // --------------------------------------------------------------------------------
    // or_else: This provides a fallback operation that might also fail.
    // The or_else method is used to handle an Err case by trying an alternative operation.
    // If the initial Result is Ok, or_else does nothing and returns the Ok value.
    // If it’s Err, it calls the closure with the error value.
    // Crucially, the closure passed to or_else must itself return a Result<T, E> of the same type.
    // This allows you to provide a fallback that could either succeed (Ok) or produce a different error (Err):

    // First attempt fails to parse.
    let first_attempt: Result<i32, _> = "hello".parse();
    // Second attempt will succeed.
    let second_attempt: Result<i32, _> = "42".parse();
    // Use or_else to try the second attempt if the first one fails.
    // The closure `|_| second_attempt` is called because first_attempt is Err.
    // It returns the second_attempt Result.
    let final_result = first_attempt.or_else(|_| second_attempt);
    println!("Final result after fallback: {:?}", final_result); // Prints: Ok(42)
}
