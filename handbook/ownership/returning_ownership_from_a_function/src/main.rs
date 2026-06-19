fn main() {
    let mut my_string = String::from("Hello, Rust!");
    my_string = return_ownership(my_string);
    println!("{}", my_string); // Now this is valid
}
fn return_ownership(s: String) -> String {
    String::from("Hello from the returned ownership")
}
