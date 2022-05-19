fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6,7];
    let slice = &arr[1 .. 5]; // [1, 2, 3, 4] "[) slice manner" 

    println!("Slice: {:?}", slice);

    println!("after function call");

    borrowing_slices(arr, slice);
}

fn borrowing_slices(arr: [u8; 8], slice: &[u8] ) {
    println!("full arr: {:?}", arr);
    println!("slice: {:?}", slice);

    println!("slice length: {}", slice.len());
    println!("0th = {}, 1st = {}", slice[0], slice[1]);

}