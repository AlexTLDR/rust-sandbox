enum Message {
    Quit,                    // Variant with no associated data
    ChangeColor(u8, u8, u8), // Variant holding three u8 values (like a tuple)
    Move { x: i32, y: i32 }, // Variant holding named fields (like a struct)
    Write(String),           // Variant holding a single String
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Received Quit message: The program should terminate"),
        // Destructure the tuple-like data directly in the pattern
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        // Destructure the struct-like data directly in the pattern
        Message::Move { x, y } => println!("Moving to coordinates {} and {}", x, y),
        // Bind the contained String to the variable 'text'
        Message::Write(s) => println!("Message content is {}", s),
    }
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::ChangeColor(23, 177, 99);
    let msg3 = Message::Move { x: 199, y: 345 };
    let msg4 = Message::Write(String::from("Enums are versatile"));

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}
