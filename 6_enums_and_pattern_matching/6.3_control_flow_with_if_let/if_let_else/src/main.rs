#[derive(Debug)]
enum UsState {
    Alabama
}

enum Coin {
    Quarter(UsState)
}

fn main() {
    let mut count = 0;
    let state = UsState::Alabama;
    let coin = Coin::Quarter(state);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
