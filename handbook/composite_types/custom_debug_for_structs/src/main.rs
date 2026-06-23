// #[derive(Debug)] standard debug implementation
use std::fmt;
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User{{username: {}, email: {}, sign_in_count: {}, active: {} }}",
            self.username, self.email, self.sign_in_count, self.active
        )
    }
}

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("me@eample.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{:?}", user1);
}
