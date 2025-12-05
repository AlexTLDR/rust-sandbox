use authentication::{login, read_line, LoginAction};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin logged in"),
                    authentication::LoginRole::User => println!("User logged in"),
                }
                break;
            }
            Some(LoginAction::Denied) => {}
            None => {
                println!("New user system");
        }

        }
        println!("Invalid credentials. Try again.");
        tries += 1;
        if tries >= 3 {
            println!("Too many failed attempts. Exiting.");
            break;
        }
    }
}
