use futures::executor::block_on;
use futures::join;

async fn say_hello() {
    println!("Hello");
    join!(second_function(), say_goodbye());
}

async fn second_function() {
    println!("Hello again");
}

async fn say_goodbye() {
    println!("Goodbye!");
}

fn main() {
    block_on(say_hello());
}
