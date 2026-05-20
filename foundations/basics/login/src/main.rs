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
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            LoginAction::Denied => {
                // Do nothing
            }
        }

        println!("Incorrect username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many failed login attempst");
            break;
        }
    }
}
