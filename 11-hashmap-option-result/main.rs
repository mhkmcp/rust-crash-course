use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    map.insert(0, "Zero");
    map.insert(1, "One");
    map.insert(2, "Two");

    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map);
}