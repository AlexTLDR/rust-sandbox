use authentication::{login, read_line, LoginAction};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            LoginAction::Granted(role) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin logged in"),
                    authentication::LoginRole::User => println!("User logged in"),
                }
                break;
            }
            LoginAction::Denied => {}
        }

        tries += 1;
        println!("Invalid credentials. Try again.");
        if tries >= 3 {
            println!("Too many failed attempts. Exiting.");
            break;
        }
    }
}
