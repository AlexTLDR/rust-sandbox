struct File(String);

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<String> for File {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

fn main() {
    let my_file = File(String::from("I am a file contents"));
    let my_string = String::from("I am a file contents");

    println!("{}", my_file.0 == my_string); // This is using Display
    println!("{}", my_file == my_string); // This is using PartialEq
}
