#[derive(Debug, PartialEq)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // Shorthand for email: email
        username, // Shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("alex@example.com"),
        username: String::from("alex"),
        sign_in_count: 1,
    };
    println!(
        "Created user with name: {} and email email: {}",
        user1.username, user1.email
    );

    let user_email = String::from("shorthand@example.com");
    let user_name = String::from("shorthand_user");
    let mut user2 = build_user(user_email, user_name);
    println!(
        "User 2 active status: {} and name is {}",
        user2.active, user2.username
    );
    user1.active = false;
    println!("Deactivated user {}", user1.username);
    user2.sign_in_count += 1;
    println!(
        "User: {} just signed in this many times: {}",
        user2.username, user2.sign_in_count
    );
}
