fn main() {
    // for loop
    println!("For");
    for x in 0 .. 6 {
        println!("{}", x);
    }

    // while loop
    println!("While");
    let mut i: u8 = 1;
    while i < 6 {
        println!("{}", i);
        i += 1;
        if i == 4 {
            println!("Let's break!");
            break; // break outa loop
            // continue to skip 
        }
    }
    
}