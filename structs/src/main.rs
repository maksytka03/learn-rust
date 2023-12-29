struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(
        String::from("someuser@example.com"),
        String::from("imauser123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // fields not explicitly set should have the same values as user1.
    }; // user1 is no longer valid because of the partial move to user2.

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        /* we only need to write the variable name since it has the same name as the field in User. */
        email,
        sign_in_count: 1,
    }
}
