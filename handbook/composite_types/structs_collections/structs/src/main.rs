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
    /*
    When using the struct update syntax,
    it’s important to remember that Rust’s ownership rules apply to each field individually.
    The behavior depends on whether a type implements the Copy trait.
    */
    let user3 = User {
        // if username or email wouldn't be explicit declared, but left under the ..user1
        // let's say
        // email: String::from("oreo@gmail.com"),
        // ..user1
        // user1 would have lost ownership of username, as String does not implement copy
        // since bool and u64 implement copy, user1 keeps ownership of those
        username: String::from("Oreo"),
        email: String::from("oreo@gmail.com"),
        ..user1
    };
    println!("The Third's user details are {:?}", user3);

    /*
    If we need to keep the original struct valid after the update,
    we must explicitly clone its non-Copy fields to create a deep copy for the new instance.
    */
    let user4 = User {
        email: String::from("Sasha"),
        username: user3.username.clone(),
        ..user3
    };
}
