
// None => failure or lack of value
// Some(value) => a tuple struct that wraps a value with type T. 
fn divide(divident: i32, divisor: i32) -> Option<i32> {
    if divident % divisor != 0 {
        None
    } else {
        Some(divident / divisor)
    }
}


fn main() {

    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwrapping a "Some" variant will extract the value wrapped.
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a "None" variant will "panic"
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}