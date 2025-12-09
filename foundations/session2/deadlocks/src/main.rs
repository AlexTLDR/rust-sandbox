use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);
fn main() {
    //let my_shared = Mutex::new(0);

    let lock = MY_SHARED.lock().unwrap();
    std::mem::drop(lock);
    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("Got the lock");
    } else {
        println!("No lock");
    }
}
