fn main() {
    let condition = true;

    // Arms need to be of the same type
    let number = if condition { 5 } else { 6 };

    // Causes mismatched types error:
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
