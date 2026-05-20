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
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                // Do nothing
            }
            None => println!("New user system"),
        }

        println!("Incorrect username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many failed login attempst");
            break;
        }
    }
}
