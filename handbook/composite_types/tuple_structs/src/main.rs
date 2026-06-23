// Define tuple structs
struct Color(u8, u8, u8); // RGB
struct Point(i32, i32); // 2D coordinates
struct Dimensions(f64, f64, f64);

fn calculate_volume(dimensions: Dimensions) -> f64 {
    dimensions.0 * dimensions.1 * dimensions.2
}

fn main() {
    // Instantiate like tuples, but with the type name
    let red = Color(255, 0, 0);
    let origin = Point(0, 0);
    // Access fields using dot notation and index
    println!("Red's green component: {}", red.1); // Accesses the second field (index 1)
    // They define distinct types
    // let point_tuple: (i32, i32) = origin; // Error: mismatched types Point != (i32, i32)
    // Can be destructured
    let Point(x, y) = origin;
    println!("Origin coordinates: x={}, y={}", x, y);

    let box_dimensions = Dimensions(30.0, 20.0, 15.5);
    println!(
        "The box dimensions are  -> length {}, width {} and height {}",
        box_dimensions.0, box_dimensions.1, box_dimensions.2
    );
    println!(
        "The volume of the box is {}",
        calculate_volume(box_dimensions)
    );
}
