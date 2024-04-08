fn main() {
    println!("Hello, world!");
    let x: i32 = add_numbers(4, 7);
    println!("{x}");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}
