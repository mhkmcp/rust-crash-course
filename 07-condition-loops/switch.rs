fn main()
{
    let num = 7;

    match num {
        0 => println!("Zero"),
        1 | 2 => println!("One or Two"),
        3..=7 => println!("3 to 7"),    // inclusive
        _ => println!("Default")
    }
}