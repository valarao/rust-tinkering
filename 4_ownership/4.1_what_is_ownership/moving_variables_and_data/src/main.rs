fn main() {
    // Copying primitive values
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // s1 gets "moved" to s2
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {}", s2);
}
