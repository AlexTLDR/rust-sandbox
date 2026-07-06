fn find_and_operate(slice: &[i32], operation: fn(i32) -> Option<i32>) -> Option<i32> {
    let found_element = slice.iter().find(|&&x| x > 10)?;
    let operation_result = operation(*found_element)?;
    Some(operation_result)
}

fn double_if_even(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n * 2) } else { None }
}

fn main() {
    let numbers1 = [5, 12, 8, 15, 6];
    let numbers2 = [5, 8, 13, 9];
    let numbers3 = [1, 2, 3];

    println!(
        "Operation result on {:?}: {:?}",
        numbers1,
        find_and_operate(&numbers1, double_if_even)
    );
    println!(
        "Operation result on {:?}: {:?}",
        numbers2,
        find_and_operate(&numbers2, double_if_even)
    ); // Output: None (because 13 is odd)
    println!(
        "Operation result on {:?}: {:?}",
        numbers3,
        find_and_operate(&numbers3, double_if_even)
    ); // Output: None (no number > 10 found)
}
