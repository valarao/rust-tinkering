struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("username: {}, email: {}, active: {}, sign_in_count: {}", user.username, user.email, user.active, user.sign_in_count);
}

fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    let user1 = build_user(String::from("otherusername123"), String::from("otheremail@example.com"));
    print_user(&user1);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    print_user(&user2);
}
