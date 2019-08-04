struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



fn main() {
    let mut user1 = User{
        email: String::from("stefan@gmail.com"),
        username: String::from("stefan"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}