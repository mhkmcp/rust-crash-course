fn main() {

    let tup1: (u8, bool, f32) = (23, false, 2.34);

    println!("tupple first {}, second {}, third {}", tup1.0, tup1.1, tup1.2);

    println!("tuple {:?}", tup1);

    let (a, b, c) = tup1;

    println!("tupple first {}, second {}, third {}", a, b, c);
}