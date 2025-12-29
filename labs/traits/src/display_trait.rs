// Run with: cargo run -p traits --bin display
use std::fmt;

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is actually a dog age {}.", self.name, self.age)
    }
}

fn main() {
    let sushi = Cat {
        name: "Sushi".to_string(),
        age: 4,
    };
    println!("{}", sushi);
}
