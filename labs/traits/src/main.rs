mod display_trait;
mod from_trait;
mod complex_example;

struct Animal {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self);
    fn run(&self);
}

impl DogLike for Animal {
    fn bark(&self) {
        println!("{} the animal is barking like crazy!", self.name)
    }
    fn run(&self) {
        println!("{} the animal is running!", self.name);
    }
}
impl DogLike for Parrot {
    fn bark(&self) {
        println!("{} the parrot is talking!", self.name);
    }
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

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
