fn main() {
    let n = double(5);
    println!("{n}");
    let i = 5;
    let n = if i == 5 { 6 } else { 7 };
    println!("{n}");

    let mut name = "Alex".to_string();
    change_name(&mut name);
    greet(&name); // without referencing, name is consumed
    greet(&name);
}
fn change_name(s: &mut String) {
    *s = format!("{s}TLDR");
}
fn greet(s: &String) {
    println!("Hello {s}")
}
fn double(n: i32) -> i32 {
    n * 2
}
