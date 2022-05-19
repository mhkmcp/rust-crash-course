fn main() {
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("len = {}", vec.len());
    println!("0th = {}", vec[0]);
    vec.push(6);
    vec.remove(0);
    println!("vector: {:?}", vec);
    vec.push(9);
    println!("len = {}", vec.len());
}