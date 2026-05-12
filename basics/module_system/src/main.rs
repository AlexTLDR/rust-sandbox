use module_system::english;
use module_system::romanian::argou;
use module_system::spanish;
use rand::RngExt;

fn main() {
    english::greet();
    spanish::greet();
    argou::cocalareala();
    let mut rng = rand::rng(); // returns a ThreadRng

    //let n: u32 = rng.random();                  // random u32
    let n_range: i32 = rng.random_range(1..=100); // random i32 between 1 and 10
    let b: bool = rng.random_bool(0.5); // true 50% of the time

    println!(" {n_range}, {b}");
}
