use std::collections::HashMap;
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let x = v.pop();
    println!("{}{:?}", v[1], x);

    let mut v = vec![1, 2, 3, 4, 5];
    println!("{}{:?}", v[0], v.get(10));

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(1, true);
    h.insert(2, false);
    println!("{:?}", h.get(&1));
}
