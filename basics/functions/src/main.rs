#![allow(unused_variables)]

fn main() {
    let number: f64 = 3.989;

    inspect_integer(number);

    let answer = 42;

    println!("The answer is {}", answer);
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    let sum = add(number as i32, answer);
    println!("{} + {} = {}", number, answer, sum);

    let countdown: i32; // declares countdown, but doesn't initialize it
    if answer < 100 {
        countdown = 10;
    } else {
        println!("The answer is clearly wrong.");
        // set countdown to some value here
        countdown = 0;
    }
    println!("The countdown begins at {}", countdown);

    let converted_to_int = convert_to_integer(23.0);
    println!("23.0 converted to integer is {}", converted_to_int);
}

fn inspect_integer(x: f64) {
    println!("The integer is {}", x);
}

fn convert_to_integer(num: f64) -> i32 {
    // For more information on using `as` to cast between numeric types, see:
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
    num.round() as i32
}
