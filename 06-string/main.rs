fn main() {

    let str: &str = "hello world";
    let mut string: String = String::from("Hello World!");

    println!("str: {}", str);
    println!("string: {}", string);

    string.push('@');
    string.push_str("23adjk");

    let slice = &string[.. 6];

    println!("slice: {}", slice);

    println!("string again: {}", string); 

    string = string.replace("Hello", "Bye");

    println!("replaced string: {}", string);
}