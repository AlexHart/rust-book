fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("test@example.com");
    user1.username = String::from("hartformer");

    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active);

    let user2 = build_user(
        String::from("ahartlhoner@gmail.com"),
        String::from("ahartlohner"),
    );
    println!(
        "{} - {} - {}",
        user2.username, user2.email, user2.sign_in_count
    );

    let user2 = User {
        sign_in_count: user2.sign_in_count + 1,
        ..user2
    };
    println!(
        "{} - {} - {}",
        user2.username, user2.email, user2.sign_in_count
    );

    let point1 = Point(23, 33, 45);
    println!("x: {}\ty: {}\tz: {}", point1.0, point1.1, point1.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
