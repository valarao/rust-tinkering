fn main() {
    let x = five();

    println!("The value of x is: {}", x);
    println!("The value of x + 1 is: {}", plus_one(x));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
