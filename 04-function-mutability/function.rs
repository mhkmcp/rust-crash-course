fn main() {

    println!("is even {}", is_even(24));
    println!("is even {}", is_even(13));


}

pub fn is_even(num: u8)->bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}