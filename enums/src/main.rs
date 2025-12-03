fn find_user<'a>(users: &'a [(u32, &'a str)], id: u32) -> Option<&'a str> {
    for (uid, name) in users {
        if *uid == id {
            return Some(*name);
        }
    }
    None
}

fn main() {
    let users = vec![(1, "Alice"), (2, "Bob")];

    // match
    match find_user(&users, 2) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    // if let
    if let Some(name) = find_user(&users, 1) {
        println!("Welcome, {}", name);
    }

    // combinators: map + unwrap_or to produce a greeting or default
    let greeting = find_user(&users, 3)
        .map(|n| format!("Hello, {}", n))
        .unwrap_or("Hello, Guest".to_string());
    println!("{}", greeting);
}
