fn find_first_greater_than_ten(slice: &[i32]) -> Option<i32> {
    slice.iter().copied().find(|&x| x > 10)
}

fn sum_first_and_last_even(slice: &[i32]) -> Option<i32> {
    let first_even = slice.iter().copied().find(|&x| x % 2 == 0)?;
    let last_even = slice.iter().copied().rfind(|&x| x % 2 == 0)?;
    Some(first_even + last_even)
}

fn main() {
    let my_slice = [3, 1, 6, 8, 2, 25, 18, 6, 90, 77, 54, 89, 192, 785, 87];
    let odd_numbers = [1, 3, 5, 17, 25];

    println!(
        "The first greater than 10 number in slice {:?} is {:?}",
        odd_numbers,
        find_first_greater_than_ten(&odd_numbers)
    );
    println!(
        "The sum for the first and last even of the {:?} slice is {:?}",
        my_slice,
        sum_first_and_last_even(&my_slice)
    );
    println!(
        "The sum for the first and last even of the {:?} slice is {:?}",
        my_slice,
        sum_first_and_last_even(&odd_numbers)
    );
}
