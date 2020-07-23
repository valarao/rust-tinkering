fn main() {
    let v0 = vec![100, 32, 57];
    for i in &v0 {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
