fn main() {
    let mut name = "Alex".to_string();
    greet_borrow(&name);
    greet_borrow(&name);
    greet_borrow_mut(&mut name);
    greet(name);
}

fn greet(s: String) {
    println!("Hello {}", s);
}

fn greet_borrow(s: &String) {
    println!("Hello {}", s);
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {}", s);
}
