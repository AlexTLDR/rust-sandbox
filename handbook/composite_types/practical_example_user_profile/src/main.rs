//#[derive(Debug)]
use std::fmt;
struct UserProfile {
    username: String,
    email: String,
    age: u32,
    active: bool,
    phone_number: String,
    address: String,
}

impl fmt::Debug for UserProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserProfile{{ username: {}, email: {}, age{}, active: {}, phone_number: {}, address: {} }}",
            self.username, self.email, self.age, self.active, self.phone_number, self.address
        )
    }
}

impl UserProfile {
    fn new(
        username: String,
        email: String,
        age: u32,
        active: bool,
        phone_number: String,
        address: String,
    ) -> Self {
        Self {
            username,
            email,
            age,
            active,
            phone_number,
            address,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn reactivate(&mut self) {
        self.active = true;
    }
    fn update_mail(&mut self, new_email: String) {
        self.email = new_email;
    }
}

fn main() {
    let mut user = UserProfile::new(
        String::from("Alex"),
        String::from("alex@example.com"),
        40,
        true,
        String::from("0123456789"),
        String::from("Green Terrace 72"),
    );
    println!("{:?}", user);
    user.deactivate();
    println!("User after deactivation: {:?}", user);
    user.reactivate();
    println!("User after reactivation: {:?}", user);
    user.update_mail(String::from("alex@gmail.com"));
    println!("The new user email is {}", user.email);
}
