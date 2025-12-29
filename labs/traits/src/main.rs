struct Animal {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("Running...");
    }
}

impl DogLike for Animal {}
impl DogLike for Parrot {}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    let polly = Parrot {
        name: "Polly".to_string(),
    };

    rover.bark();
    rover.run();
    polly.bark();
    polly.run();
}
