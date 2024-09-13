pub fn main() {
    let mut my_slice: &str = "This is a string slice!";
    println!("{}", my_slice);
    my_slice = "Update value is string slice 2";
    println!("After change: {}", my_slice);
}