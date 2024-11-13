
// Similar to C
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Field init shorthand when the field names are exactly the same as in the struct
// Helps with redundancy in also needing to keep track of username: username, email: email etc
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Struct fields do not need to be named
struct Color(i32, i32, i32);
//struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

struct Point { x: i32, y: i32 }

// To note, if you want to make an element of the struct mutable, the whole struct must be
// mutable!!
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // How to change the info of a field if it is mutable
    user1.email = String::from("anotheremail@example.com");

    // Instead of doing this to copy some fields, changing others
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Use this updating syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1     // Must be last to specify all remaining fields not explicitly called
    };
    // To note, user1 can no longer be used due to ownership
    
    let black = Color(0, 0, 0);
    //let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    *x += 1;
    println!("{}, {}", p.x, p.y);
}
