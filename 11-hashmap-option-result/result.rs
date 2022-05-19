#[derive(Debug)]
enum MyError {
    Error1
}

// Err => an enum that contain an error code
// Ok(value) => a wrapper that contains a value. 
fn divide(divident: i32, divisor: i32) -> Result<i32, MyError> {
    if divident % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(divident / divisor)
    }
}


fn main() {

    let divide1= divide(3, 2);
    // let res = divide1.expect("Division Crashed!");

    // match divide1 {
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }

    // if divide1.is_ok() {
    //     println!("{}", divide1.unwrap());
    // }
    // println!("{}", divide1.unwrap_or(100)); 

    // println!("{}", res);
}