struct Marker; // A unit-like struct
trait MyTrait {
    fn description(&self) -> &'static str;
}
// We can implement traits for unit-like structs
impl MyTrait for Marker {
    fn description(&self) -> &'static str {
        "This is a marker instance."
    }
}
fn main() {
    let m = Marker; // Create an instance (it holds no data)
    println!("{}", m.description()); // Call method from the implemented trait
}
