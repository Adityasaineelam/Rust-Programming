// Structs must be defined outside the main function
struct Book {
    title: String,
    author: String,
    pages: u32,
    availability: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    signin_account: u64,
}

fn main() {
    let rect: (i32, i32) = (200, 500);

    // You can now create instances of Book and User here
    let my_book = Book {
        title: String::from("Rust Essentials"),
        author: String::from("Jane Doe"),
        pages: 320,
        availability: true,
    };

    let user1 = User {
        active: true,
        username: String::from("rustacean"),
        email: String::from("rust@lang.org"),
        signin_account: 123456789,
    };

    println!("Rectangle dimensions: {:?}", rect);
    println!("Book title: {}", my_book.title);
    println!("User email: {}", user1.email);
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("Black RGB: {}, {}, {}", black.0, black.1, black.2);
    println!("White RGB: {}, {}, {}", white.0, white.1, white.2);
    println!("Origin Point: {}, {}, {}", origin.0, origin.1, origin.2);
}