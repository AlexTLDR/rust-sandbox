use std::fs;

fn read_content(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn main() {
    println!("Calling read_content for an existing file");
    fs::write("exists.txt", "This file exists and has some content");

    match read_content("exists.txt") {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => eprintln!("File not found"),
            std::io::ErrorKind::PermissionDenied => eprintln!("Permission denied"),
            _ => eprintln!("Another error: {}!", e),
        },
    }

    match read_content("doesnot_exist.txt") {
        Ok(s) => println!("This file doesn't exist, should not print this"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => eprintln!("File not found"),
            std::io::ErrorKind::PermissionDenied => eprintln!("Permission denied"),
            _ => eprintln!("Another error: {}!", e),
        },
    }
    fs::remove_file("exists.txt");
}
