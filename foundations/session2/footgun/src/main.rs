static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        let final_val = COUNTER;
        println!("Final counter value: {}", final_val);
    }
}
