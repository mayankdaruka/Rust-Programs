#[derive(Debug)] // allows pretty printing structs with println!
enum IPAddrType {
    V4,
    V6,
}

#[derive(Debug)] // allows pretty printing structs with println!
struct IPAddr {
    kind: IPAddrType,
    address: String,
}

#[derive(Debug)] // allows pretty printing structs with println!
enum IPAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)] // allows pretty printing structs with println!
enum IPAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)] // allows pretty printing structs with println!
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // has named fields like a struct does
    Write(String), // single string
    ChangeColor(i32, i32, i32), // 3 values
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("the value of x is {:?}", self);
    }
}

enum Option<T> {
    None,
    Some(T),
}

/** doing it the way above is similar to doing it this way:
 * struct QuitMessage; // unit struct
 * struct MoveMessage {
 *  x: i32,
 *  y: i32,
 * }
 * struct WriteMessage(String); // tuple struct
 * struct ChangeColorMessage(i32, i32, i32); // tuple struct
 */
 
fn main() {
    let four = IPAddrType::V4;
    let six = IPAddrType::V6;
    println!("{:?}", four);

    let home = IPAddr {
        kind: four,
        address: String::from("127.0.0.1")
    };
    println!("{:?}", home);

    let home = IPAddr2::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let home = IPAddr3::V4(127, 0, 0, 1);
    println!("{:?}", home);

    let message_move = Message::Move { x: 15, y: 10 };
    message_move.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // generates error - no implementation for i8 + Option<i8>
}