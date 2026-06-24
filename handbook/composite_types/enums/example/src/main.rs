// Define an enum for different kinds of web events
enum WebEvent {
    PageLoad,                 // Simple variant with no data
    PageUnload,               // Simple variant with no data
    KeyPress(char),           // Tuple-like variant holding a char
    Click { x: i64, y: i64 }, // Struct-like variant holding named data
}
// A function to process different web events
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // Destructure the data from the variant
        WebEvent::KeyPress(c) => println!("Key pressed: '{}'.", c),
        WebEvent::Click { x, y } => println!("Clicked at coordinates: x={}, y={}.", x, y),
    }
}
fn main() {
    let load_event = WebEvent::PageLoad;
    let unload_event = WebEvent::PageUnload;
    let click_event = WebEvent::Click { x: 20, y: 80 };
    let key_event = WebEvent::KeyPress('x');
    inspect(load_event);
    inspect(unload_event);
    inspect(click_event);
    inspect(key_event);
}
