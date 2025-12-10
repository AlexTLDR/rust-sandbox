fn main() {
    // Explicitly size pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from a custom Rayon thread pool!"));

    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from scoped thread number {}", n);
            })
        }
    });
    println!("Hello from the main thread");
}
