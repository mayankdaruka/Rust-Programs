struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)] // allows pretty printing structs with println!
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("user1's details: {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    // user1.username = String::from("mayankdaruka"); // generates error

    let mut user2 = User {
        email: String::from("mayankdaruka@example.com"),
        ..user1
    };

    println!("user2's username: {}", user2.username);
    user2.username = String::from("mayankdaruka");

    // we can use tuple structs without named fields to create different types
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

    println!("first value of black: {}",  black.0);

    let rect1 = Rectangle {
        height: 20,
        width: 15,
    };

    println!("area of rect1 is: {}", calculate_area(&rect1));

    // use {:?} for pretty-print along with #[derive(Debug)]
    println!("rect1 : {:?}", rect1);

    // takes ownership of the expression, prints file
    // and line number of macro call and resulting value of expression, then returns ownership of value
    dbg!(&rect1);
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}