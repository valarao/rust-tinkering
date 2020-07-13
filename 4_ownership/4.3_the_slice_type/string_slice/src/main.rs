fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let slice0 = &s[0..2];
    let slice1 = &s[..2];

    println!("{}, {}", slice0, slice1);

    let len = s.len();
    let slice0 = &s[3..len];
    let slice1 = &s[3..];
    println!("{}, {}", slice0, slice1);
}
