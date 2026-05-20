use crate::LoginRole::{Admin, User};

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}
#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}
pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();

    if username == "admin" && password == "password" {
        LoginAction::Granted(Admin)
    } else if username == "bob" && password == "password" {
        LoginAction::Granted(User)
    } else {
        LoginAction::Denied
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Alex", greet_user("Alex"))
    }

    #[test]
    fn test_login() {
        assert_eq!(login("Admin", "password"), LoginAction::Granted(Admin));
        assert_eq!(login("admin", "password"), LoginAction::Granted(Admin));
        assert_eq!(login("bob", "password"), LoginAction::Granted(User));
        assert_eq!(login("admin", "notcorrectpassword"), LoginAction::Denied);
    }
}
