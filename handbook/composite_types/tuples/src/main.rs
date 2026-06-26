fn main() {
    let basic_tuple: (i32, f64, bool) = (100, std::f64::consts::PI, true);
    let mix_tuple = ("Rust", 2024, '🦀');
    println!("Basic tuple: {:?}", basic_tuple);
    println!("Mixed tuple: {:?}", mix_tuple);

    let first_val = basic_tuple.0;
    let second_val = basic_tuple.1;
    println!("First is {} and second is {}", first_val, second_val);

    // destructuring
    let (language, year, mascot) = mix_tuple;
    println!("Language: {}, year: {}, mascot: {}", language, year, mascot);
    // Ownership: If mix_tuple contained non-Copy types (like String),
    // destructuring would move ownership into 'language', 'year', 'mascot'.
    // The original tuple might become unusable.

    let (sum, difference, product) = calculate(23, 67);
    println!(
        "Sum is {}, difference is {} and product is {}",
        sum, difference, product
    );

    // Destructuring in a loop
    let points = vec![(0, 0), (1, 2), (3, 4)];
    for (x, y) in points {
        println!("Point at ({}, {})", x, y);
    }

    let tripple_point1 = EuclideanPoint::ThreeDimension(0.0, 0.0, 0.0);
    let tripple_point2 = EuclideanPoint::ThreeDimension(1.2, 3.4, 5.6);
    println!(
        "The distance between the tripple points is {}",
        distance(tripple_point1, tripple_point2)
    );
}

fn calculate(a: i32, b: i32) -> (i32, i32, i32) {
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    (sum, difference, product)
}

enum EuclideanPoint {
    TwoDimension(f64, f64),
    ThreeDimension(f64, f64, f64),
}
fn distance(point1: EuclideanPoint, point2: EuclideanPoint) -> f64 {
    match (point1, point2) {
        (EuclideanPoint::TwoDimension(x1, y1), EuclideanPoint::TwoDimension(x2, y2)) => {
            ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
        }
        (
            EuclideanPoint::ThreeDimension(x1, y1, z1),
            EuclideanPoint::ThreeDimension(x2, y2, z2),
        ) => ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt(),
        _ => panic!("Mixed 2D/3D points are not supported"),
    }
}
