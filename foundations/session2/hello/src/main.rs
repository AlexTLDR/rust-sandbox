fn hello_thread(n: u32) {
    println!("Hello from a thread {n}!");
}

fn main() {
    println!("Hello from the main thread");

    let mut thread_handles = vec![];
    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|h| h.join().unwrap());
}
