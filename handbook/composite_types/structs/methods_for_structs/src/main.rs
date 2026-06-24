#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    } // not a method but an associated function
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    } // not a method but an associated function
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn has_valid_width(&self) -> bool {
        self.width > 0
    }
    fn double_width(&mut self) {
        self.width *= 2;
    }
}

fn main() {
    let mut rect1 = Rectangle::new(30, 50);

    println!("The area of rect1 is {}", rect1.area());
    println!("Does rect1 have a valid width? {}", rect1.has_valid_width());
    println!("Original width: {}", rect1.width);
    rect1.double_width();
    println!("Width after doubling: {}", rect1.width);
    println!("New area after doubling: {}", rect1.area());

    let square_rect = Rectangle::square(45);
    println!(
        "The square is {:?} and has an area of {}",
        square_rect,
        square_rect.area()
    );
}
