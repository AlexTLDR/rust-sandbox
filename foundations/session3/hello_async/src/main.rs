use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something_sync() {
    println!("Not async!")
}
async fn say_hello() {
    println!("Hello from async function!");
    join!(second_function(), say_goodbye());
    let result = double(10).await;
    println!("Double of 10 is: {}", result);

    let futures = vec![double(1), double(2), double(3), double(4), double(5)];
    let results: Vec<u32> = join_all(futures).await;
    println!("{results:?}");
    do_something_sync();
}

async fn second_function() {
    println!("Hello from the second async function!");
}

async fn say_goodbye() {
    println!("Goodbye from async function!");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello());
}
