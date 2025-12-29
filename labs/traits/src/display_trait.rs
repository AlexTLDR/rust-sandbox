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

fn print_barks(input: String) {
    println!("{} She barks at everybody!", input);
}

fn main() {
    let sushi = Cat {
        name: "Sushi".to_string(),
        age: 4,
    };
    println!("{}", sushi);
    print_barks(sushi.to_string());
    println!(
        "Using .chars().count() to count the number of characters in the string: {}",
        sushi.to_string().chars().count()
    );
}
