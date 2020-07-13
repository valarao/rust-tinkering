fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // [2, 3]

    for &item in slice {
        println!("{}", item);
    }
}
