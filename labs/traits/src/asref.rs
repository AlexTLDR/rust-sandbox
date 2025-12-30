fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}

fn main() {
    print_it("Print me, I am a string literal.");
    print_it(String::from("Print me, I am a string."));
}
