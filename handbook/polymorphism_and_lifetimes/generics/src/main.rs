fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We restrict T: It must be comparable (PartialOrd) and copyable (Copy)
// We return Option<T> to handle the case where the list is empty.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let integers = vec![1, 2, 3, 4, 5];
    let gen_integers = vec![1, 2, 3, 4, 5];
    let chars = vec!['a', 'b', 'c'];
    let gen_chars = vec!['a', 'b', 'c'];
    println!(
        "Biggest integers and generic integers are {} and {:?} and biggest chars and generic chars are {} and {:?}.",
        largest_i32(&integers),
        largest(&gen_integers),
        largest_char(&chars),
        largest(&gen_chars)
    )
}

/*
use std::fmt::{Debug, Display};
// Hard to read: The bounds clutter the function name
fn compare_prints<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) { }
// Easier to read: The bounds are moved to the 'where' clause
fn compare_prints<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Function body...
}
*/
