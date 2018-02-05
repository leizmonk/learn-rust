fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::("someusername123"),
        active: true,
        sign_in_count, 1,
    };

    // Struct update syntax, reference values from user1 
    // in instantiating user2 to produce identical values 
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    }

    // ..user1 struct update syntax specifies to use the
    // exact same values in user1 for all remaining fields
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirduser333"),
        ..user1
    }
}

// Function that returns a new instanc of the User struct
fn build_user(email: String, username: String) -> User {
    User {
        // Can use field init shorthand when the param and struct
        // field names are identical i.e. "email: email -> email"
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}