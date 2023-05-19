fn main() {
    let mut user = User {
        active: true,
        email: String::from("someEmail@gmail.com"),
        username: String::from("me"),
        sign_in_count: 0,
    };
    user.email = String::from("a different email");

    let user2 = User {
        active: user.active,
        username: user.username,
        email: String::from("adiffEmail@example.com"),
        sign_in_count: 1,
    };

    let user3 = User {
        username: String::from("newUser"),
        ..user
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Similar to case objects in scala
struct AlwaysEqual;
