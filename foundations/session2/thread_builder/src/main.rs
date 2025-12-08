use std::thread;

fn my_thread() {
    println!(
        "Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}

fn main() {
    thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();
}
