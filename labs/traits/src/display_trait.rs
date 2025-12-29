// Run with: cargo run -p traits --bin display

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let sushi = Cat {
        name: "Sushi".to_string(),
        age: 4,
    };
    println!("{sushi:?} is actually a dog.")
}
