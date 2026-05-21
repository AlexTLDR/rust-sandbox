fn main() {
    // let mut my_vector = Vec::new();
    // for i in 0..900 {
    //     my_vector.push(i);
    //     println!(
    //         "Size: {}, Capacity: {}",
    //         my_vector.len(),
    //         my_vector.capacity()
    //     );
    // }
    // println!("{:?}", my_vector);

    let mut my_vector = Vec::with_capacity(900);
    for i in 0..900 {
        my_vector.push(i);
        println!(
            "Size: {}, Capacity: {}",
            my_vector.len(),
            my_vector.capacity()
        );
    }
    println!("{:?}", my_vector);
}
