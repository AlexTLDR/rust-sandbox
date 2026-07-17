use std::fmt::Display;

fn get_status() -> impl Display {
    "System All Green"
}

fn get_detailed_status(checks_passed: u32) -> impl Display {
    format!("{checks_passed} checks passed - System All Green")
}

// NOTE: This feature has a limit!
// You cannot return distinct types conditionally.
// This will NOT compile because the compiler needs one concrete type hidden behind the trait.
//
// fn invalid_return(flag: bool) -> impl Display {
//     if flag {
//         "Success" // This is a &str
//     } else {
//         100       // This is an i32
//     }
// }

// Fix 1: make both branches return the SAME concrete type.
fn valid_return_same_type(flag: bool) -> impl Display {
    if flag {
        String::from("Success")
    } else {
        100.to_string()
    }
}

// Fix 2: return a trait object — distinct types allowed, at the cost of
// a heap allocation and dynamic dispatch.

fn valid_return_trait_object(flag: bool) -> Box<dyn Display> {
    if flag {
        Box::new("Success")
    } else {
        Box::new(100)
    }
}

fn main() {
    let status = get_status();
    println!("Status: {status}");
    println!("Details: {}", get_detailed_status(42));

    println!("same type, flag=true  -> {}", valid_return_same_type(true));
    println!("same type, flag=false -> {}", valid_return_same_type(false));

    println!(
        "trait object, flag=true  -> {}",
        valid_return_trait_object(true)
    );
    println!(
        "trait object, flag=false -> {}",
        valid_return_trait_object(false)
    );
}
