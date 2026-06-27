#[derive(Debug)]
struct Point(f64, f64);

#[derive(Debug)]
enum Shape {
    Circle {
        center: Point,
        radius: f64,
    },
    Rectangle {
        top_left: Point,
        width: f64,
        height: f64,
    },
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius.powi(2),
            Shape::Rectangle { width, height, .. } => width * height,
        }
    }
}
fn main() {
    let circle = Shape::Circle {
        center: Point(23.7, 65.9),
        radius: 23.99,
    };
    let rectangle = Shape::Rectangle {
        top_left: Point(29.0, 47.6),
        width: 92.87,
        height: 57.94,
    };
    let shapes = vec![circle, rectangle];
    for shape in shapes {
        println!(
            "Debug print is {:?} and the area is {:.2}",
            shape,
            shape.area()
        );
    }
}
